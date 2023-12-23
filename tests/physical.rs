#![allow(unused_imports)]

//! These tests are only useful if you have the physical board attached to your device.
//! They are ignored by default, but can be run using the feature that corresponds to the board.
//!

use std::ops::DerefMut;

use rpi_devices::{
    display_mipidsi::{func as img_func, images, pixelcolor, ByteOrder, *},
    errors::{IntoRPiResult, RPiResult},
};

use std::time::Duration;

use serial_test::serial;

use rpi_devices::logger;

const IMAGE_BIN_PATHS: [&str; 3] = [
    "tests/images/bus.bin",
    "tests/images/landscape.bin",
    "tests/images/tank.bin",
];

#[allow(dead_code)]
async fn load_bytes<'e>() -> RPiResult<'e, Vec<Vec<u8>>> {
    let mut tasks = Vec::with_capacity(IMAGE_BIN_PATHS.len());
    for path in IMAGE_BIN_PATHS {
        // This call will make them start running in the background
        // immediately.
        tasks.push(tokio::spawn(img_func::fs::read_bytes_from_file(path)));
    }

    let mut outputs = Vec::with_capacity(tasks.len());
    for task in tasks {
        outputs.push(task.await.into_rpi_result()?.into_rpi_result()?)
    }

    Ok(outputs)
}

#[allow(dead_code)]
fn load_raw<'a, 'e, COLOUR, const W: u16>(bytes_array: &'a [Vec<u8>]) -> Vec<ImageRaw<'a, COLOUR>>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    ImageRaw<'a, COLOUR>: ImageDrawable<Color = COLOUR>,
{
    bytes_array
        .iter()
        .map(|bytes| img_func::image_conversions::raw_from_bytes(bytes.as_slice(), W as u32))
        .collect()
}

#[allow(dead_code)]
fn load_images<'a, 'e, COLOUR, const W: u16>(
    raws: &'a [ImageRaw<'a, COLOUR>],
) -> Vec<Image<'a, ImageRaw<'a, COLOUR>>>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    for<'i> ImageRaw<'i, COLOUR>: ImageDrawable<Color = COLOUR>,
{
    raws.iter()
        .map(|raw| img_func::image_conversions::image_from_raw(raw, 0, 0))
        .collect()
}

#[cfg(feature = "pimoroni-display-hat-mini")]
mod pimoroni_display_hat_mini {
    use super::*;
    use rpi_devices::{
        boards::PimoroniDisplayHATMini,
        gpio::{func, Button, DisplayBacklight, RgbLed},
    };
    use rpi_display_mipidsi::traits::BacklightComponent;

    #[tokio::test]
    async fn physical_press() {
        let gpio = func::init_gpio().unwrap();

        logger::debug("\x1b[38;5;11mPress a button on the Pimoroni Display HAT Mini...\x1b[39m");
        macro_rules! expand_buttons {
            ($((
                $name:ident,
                $pin:expr$(,)?
            )),*$(,)?) => {
                $(
                    let $name = Button::new(&gpio, $pin);
                )*

                tokio::select! {
                    $(
                        result = $name.pressed_and_released(None) => if result.unwrap_or(false) {
                            logger::info(&format!("Pressed {}", stringify!($name)));
                        } else {
                            logger::warning(&format!("Timeout {} or error", stringify!($name)));
                        },
                    )*
                }
            };
        }

        expand_buttons!(
            (button_a, PimoroniDisplayHATMini::BUTTON_A),
            (button_b, PimoroniDisplayHATMini::BUTTON_B),
            (button_x, PimoroniDisplayHATMini::BUTTON_X),
            (button_y, PimoroniDisplayHATMini::BUTTON_Y),
        );
    }

    #[tokio::test]
    #[serial]
    async fn physical_led_basic_rgb() {
        let gpio = func::init_gpio().unwrap();

        let mut led = RgbLed::new(
            &gpio,
            PimoroniDisplayHATMini::LED_R,
            PimoroniDisplayHATMini::LED_G,
            PimoroniDisplayHATMini::LED_B,
            50.,
        );

        for rgb in &[(255, 0, 0), (0, 255, 0), (0, 0, 255)] {
            led.set_values(rgb.0, rgb.1, rgb.2).unwrap();
            tokio::time::sleep(Duration::from_millis(500)).await;
            led.disable().unwrap();
            tokio::time::sleep(Duration::from_millis(1000)).await;
            led.enable().unwrap();
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    #[tokio::test]
    #[serial]
    async fn physical_led_transition() {
        let gpio = func::init_gpio().unwrap();

        let mut led = RgbLed::new(
            &gpio,
            PimoroniDisplayHATMini::LED_R,
            PimoroniDisplayHATMini::LED_G,
            PimoroniDisplayHATMini::LED_B,
            50.,
        );

        let rgbs = [(0, 0, 32), (0, 0, 128), (192, 0, 128), (192, 192, 0)];

        for slice in rgbs.windows(2) {
            let from = slice[0];
            let to = slice[1];

            led.set_values(from.0, from.1, from.2).unwrap();
            led.transition_to(to.0, to.1, to.2, 1 << 5, Duration::from_secs(4))
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    #[serial]
    async fn physical_display_backlight() {
        let gpio = func::init_gpio().unwrap();

        let mut backlight =
            DisplayBacklight::new(&gpio, PimoroniDisplayHATMini::DISPLAY_BACKLIGHT, 50.);

        logger::debug("\x1b[38;5;11mTransitioning the backlight from OFF to 100%...\x1b[39m");
        backlight
            .transition_to(1.0, 32, Duration::from_secs(2))
            .await
            .expect("Failed to transition to 1.0");

        logger::debug("\x1b[38;5;11mDisabling the backlight for 1s...\x1b[39m");
        backlight.disable().expect("Failed to disable");
        tokio::time::sleep(Duration::from_secs(1)).await;

        logger::debug("\x1b[38;5;11mRe-enabling the backlight for 1s...\x1b[39m");
        backlight.enable().expect("Failed to enable");
        tokio::time::sleep(Duration::from_secs(1)).await;

        logger::debug("\x1b[38;5;11mTransitioning the backlight from 100% to OFF...\x1b[39m");
        backlight
            .transition_to(0., 32, Duration::from_secs(2))
            .await
            .expect("Failed to transition to 0.");
    }

    #[tokio::test]
    #[serial]
    async fn physical_display_init() {
        let unit = PimoroniDisplayHATMini::init().expect("Failed to initialize Display HAT Mini.");

        async {
            let mut display = unit.display.lock().await;

            display
                .backlight
                .transition_to(1., 32, Duration::from_secs(2))
                .await
                .expect("Failed to transition backlight to full power.");

            display.fill_white().expect("Failed to fill display white.");
            tokio::time::sleep(Duration::from_secs(1)).await;

            display.fill_red().expect("Failed to fill display red.");
            tokio::time::sleep(Duration::from_secs(1)).await;

            display.fill_green().expect("Failed to fill display green.");
            tokio::time::sleep(Duration::from_secs(1)).await;

            display.fill_blue().expect("Failed to fill display blue.");
            tokio::time::sleep(Duration::from_secs(1)).await;

            display
                .backlight
                .transition_to(0., 32, Duration::from_secs(2))
                .await
                .expect("Failed to transition backlight to dark.");
        }
        .await;
    }

    #[tokio::test]
    #[serial]
    #[cfg(feature = "transitions")]
    async fn physical_display_images() {
        let unit = PimoroniDisplayHATMini::init().expect("Failed to initialize Display HAT Mini.");

        async {
            let mut lcd = unit.display.lock().await;
            let bytes_array = load_bytes().await.expect("Failed to load bytes.");
            let raws = load_raw::<pixelcolor::Rgb565, 320>(&bytes_array);

            lcd.fill_black().expect("Failed to fill display black.");

            const STEPS: u32 = 40;

            for (id, raw) in raws.into_iter().enumerate() {
                if id == 0 {
                    lcd.draw_image(&img_func::image_conversions::image_from_raw(&raw, 0, 0))
                        .expect("Failed to draw image.");

                    // For the first image, once we have loaded the image to the screen, we
                    // fade in.
                    lcd.backlight
                        .transition_to(1., 32, Duration::from_secs(2))
                        .await
                        .expect("Failed to transition backlight to full power.");
                } else {
                    let sweeper = img_func::transitions::sweep(
                        40,
                        img_func::transitions::SweepDirection::FromLeft,
                    );

                    lcd.draw_transition_to(&raw, sweeper, STEPS, Duration::from_secs(2))
                        .await
                        .expect("Failed to transition image.");
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            lcd.backlight
                .transition_to(0., 32, Duration::from_secs(2))
                .await
                .expect("Failed to transition backlight to dark.");
            lcd.fill_black().expect("Failed to fill display black.");
        }
        .await;
    }

    #[tokio::test]
    #[cfg(feature = "bmp")]
    #[serial]
    async fn physical_display_bmp() {
        let unit = PimoroniDisplayHATMini::init().expect("Failed to initialize Display HAT Mini.");

        let mut display = unit.display.lock().await;

        let bytes = img_func::fs::read_bytes_from_file("tests/images/travel.bmp")
            .await
            .expect("Failed to load bytes.");
        let bmp = img_func::bmp::bmp_from_bytes::<pixelcolor::Rgb565>(bytes.as_slice())
            .expect("Failed to load BMP.");

        const STEPS: u32 = 40;
        let mut start_time = tokio::time::Instant::now();
        let step_duration = Duration::from_secs(6) / STEPS;

        for step in 0..STEPS {
            let target_time = start_time + step_duration;

            let sub_image = img_func::crop::crop_horizontal(&bmp, (step * 40 / STEPS) as i32, 320);

            let image = img_func::image_conversions::image_from_raw(&sub_image, 0, 0);
            display.draw_image(&image).expect("Failed to draw image.");

            if step < 32 {
                display
                    .backlight
                    .set_value(step as f64 / 32.)
                    .expect("Failed to set backlight value.");
            }

            let frame_time = tokio::time::Instant::now() - start_time;
            logger::debug(&format!(
                "Target duration: {step_duration:?}, Frame time: {frame_time:?}"
            ));
            start_time = tokio::time::Instant::now();
            tokio::time::sleep_until(target_time).await;
        }

        tokio::time::sleep(Duration::from_secs(2)).await;

        display
            .backlight
            .transition_to(0., 32, Duration::from_secs(2))
            .await
            .expect("Failed to transition backlight to dark.");
        display.fill_black().expect("Failed to fill display black.");
    }

    #[tokio::test]
    #[cfg(feature = "bmp")]
    #[cfg(feature = "transitions")]
    #[serial]
    async fn physical_transverse_bmp() {
        let unit = PimoroniDisplayHATMini::init().expect("Failed to initialize Display HAT Mini.");

        let mut lcd = unit.display.lock().await;

        lcd.backlight
            .set_value(1.)
            .expect("Failed to set backlight value.");

        let bytes = img_func::fs::read_bytes_from_file("tests/images/panorama.bmp")
            .await
            .expect("Failed to load bytes.");
        let bmp = img_func::bmp::bmp_from_bytes::<pixelcolor::Rgb565>(bytes.as_slice())
            .expect("Failed to load BMP.");

        const TRANSVERE: u32 = 463;
        const STEPS: u32 = TRANSVERE;

        let transition = img_func::transitions::transverse(STEPS, 0, 0, TRANSVERE, 0);
        img_func::transitions::Transition::new_self(
            &mut lcd.screen,
            &bmp,
            transition,
            STEPS,
            Duration::from_secs(22),
        )
        .start()
        .await
        .expect("Unable to transverse image.");

        lcd.backlight
            .disable()
            .expect("Failed to disable backlight.");
    }

    #[tokio::test]
    #[serial]
    async fn physical_owned_image_raw() {
        let image = images::OwnedImageRaw::<pixelcolor::Rgb565, BigEndian>::from_path_size(
            "tests/images/bus.bin",
            Size::new(320, 240),
        )
        .await
        .expect("Failed to load image.");

        let unit = PimoroniDisplayHATMini::init().expect("Failed to initialize Display HAT Mini.");

        unit.display
            .lock()
            .await
            .draw_image(image.image().expect("Failed to create image."))
            .expect("Failed to draw image.");

        let duration = Duration::from_secs(2);
        unit.backlight_fade_in(32, duration)
            .await
            .expect("Failed to fade in backlight.");
        tokio::time::sleep(duration).await;
        unit.backlight_fade_out(32, duration)
            .await
            .expect("Failed to fade out backlight.");
    }

    #[tokio::test]
    #[cfg(feature = "bmp")]
    #[serial]
    async fn physical_owned_bmp() {
        use rpi_display_mipidsi::images::OwnsImage;

        let image = images::OwnedBmp::<pixelcolor::Rgb565>::from_path("tests/images/panorama.bmp")
            .await
            .expect("Failed to load image.");

        let unit = PimoroniDisplayHATMini::init().expect("Failed to initialize Display HAT Mini.");

        const TRANSVERE: u32 = 463;
        const STEPS: u32 = TRANSVERE / 3;

        unit.backlight_on()
            .await
            .expect("Failed to turn on backlight.");

        let duration = Duration::from_secs(2);
        {
            let mut display = unit.display.lock().await;
            let transition = img_func::transitions::transverse(STEPS, 0, 0, TRANSVERE, 0);
            display
                .draw_transition_to(
                    image.raw().expect("Failed to create image."),
                    transition,
                    STEPS,
                    Duration::from_secs(22),
                )
                .await
                .expect("Failed to draw image.");
        }

        unit.backlight_fade_out(32, duration)
            .await
            .expect("Failed to fade out backlight.");
    }

    #[tokio::test]
    #[cfg(feature = "text")]
    #[serial]
    async fn physical_draw_text() {
        let unit = PimoroniDisplayHATMini::init().expect("Failed to initialize Display HAT Mini.");

        let mut lcd = unit.display.lock().await;

        const MARGIN: i32 = 16;

        let mut text = "The \x1b[36mST7789VW\x1b[37m is a \x1b[4msingle-chip \
        controller/driver\x1b[24m for \x1b[94m262K-color\x1b[37m, graphic type TFT-LCD. \
        It consists of 720 source line and 320 gate line driving circuits.\n\n\
        This chip is capable of connecting \
        directly to an external microprocessor, and accepts, \
        \x1b[4m8-bits/9-bits/16-bits/18-bits\x1b[24m \
        parallel interface. Display data can be stored in the on-chip display data RAM of \
        \x1b[97m240x320x18 bits.\x1b[37m\n\n\
        It can perform display data RAM read/write operation with no external operation \
        clock to minimize power consumption. In addition, because of the integrated \
        power supply circuit necessary to drive liquid crystal; \
        it is possible to make a display system with the fewest components.\n\n\
        \x1b[46mFEATURES\x1b[49m\n\n\
        \x1b[96m* Single chip TFT-LCD Controller/Driver with On-chip Frame Memory (FM)\x1b[37m\n\
        \x1b[96m* Display Resolution: 240*RGB (H) *320(V)\x1b[37m\n\
        \x1b[96m* Frame Memory Size: 240 x 320 x 18-bit = 1,382,400 bits\x1b[37m\n\
        \x1b[96m* LCD Driver Output Circuits\x1b[37m\n\
        \x1b[94m-\x1b[37m Source Outputs: 240 RGB Channels\n\
        \x1b[94m-\x1b[37m Gate Outputs: 320 Channels\n\
        \x1b[94m-\x1b[37m Common Electrode Output\n\
        \x1b[96m* Display Colors (Color Mode)\x1b[37m\n\
        \x1b[94m-\x1b[37m Full Color: 262K, RGB=(666) max., Idle Mode Off\n\
        \x1b[94m-\x1b[37m Color Reduce: 8-color, RGB=(111), Idle Mode On\n\
        \x1b[96m* Programmable Pixel Color Format (Color Depth) for Various Display Data input Format\x1b[37m\n\
        \x1b[94m-\x1b[37m 12-bit/pixel: RGB=(444)\n\
        \x1b[94m-\x1b[37m 16-bit/pixel: RGB=(565)\n\
        \x1b[94m-\x1b[37m 18-bit/pixel: RGB=(666)\n\
        \x1b[96m* MCU Interface\x1b[37m\n\
        \x1b[94m-\x1b[37m Parallel 8080-series MCU Interface (8-bit, 9-bit, 16-bit & 18-bit)\n\
        \x1b[94m-\x1b[37m 6/16/18 RGB Interface(VSYNC, HSYNC, DOTCLK, ENABLE, DB[17:0])\n\
        \x1b[94m-\x1b[37m Serial Peripheral Interface(SPI Interface)\n\
        \x1b[94m-\x1b[37m VSYNC Interface
        ".to_owned();

        lcd.fill_blue().expect("Failed to fill display blue.");
        lcd.draw_title::<20>(
            "\x1b[97mST7789\x1b[33m | LCD Display",
            pixelcolor::Rgb565::WHITE,
            Some(Point::new(0, 120 - 10)),
            None,
        )
        .expect("Failed to draw title.");
        lcd.backlight
            .transition_to(1., STEPS, Duration::from_secs_f32(0.3))
            .await
            .expect("Failed to enable backlight.");

        tokio::time::sleep(Duration::from_secs(4)).await;
        lcd.backlight
            .transition_to(0., STEPS, Duration::from_secs_f32(0.3))
            .await
            .expect("Failed to disable backlight.");

        const STEPS: u32 = 12;
        loop {
            lcd.fill_black().expect("Failed to fill display black.");

            lcd.draw_text::<20>(
                "ST7789VW",
                pixelcolor::Rgb565::CYAN,
                Some(Point::new(MARGIN, MARGIN)),
                None,
            )
            .expect("Failed to draw text.");

            text = lcd
                .draw_text::<15>(
                    &text,
                    pixelcolor::Rgb565::WHITE,
                    Some(Point::new(MARGIN, MARGIN + 20 + 4)),
                    None,
                )
                .expect("Failed to draw text.");

            lcd.backlight
                .transition_to(1., STEPS, Duration::from_secs_f32(0.3))
                .await
                .expect("Failed to enable backlight.");

            tokio::time::sleep(Duration::from_secs(4)).await;
            lcd.backlight
                .transition_to(0., STEPS, Duration::from_secs_f32(0.3))
                .await
                .expect("Failed to disable backlight.");

            if text.is_empty() {
                break;
            }
        }
    }
}

#[cfg(feature = "pimoroni-enviro-plus")]
mod pimoroni_enviro_plus {
    use super::*;
    use rpi_devices::boards::PimoroniEnviroPlus;

    #[tokio::test]
    #[serial]
    async fn physical_display_init() {
        let unit = PimoroniEnviroPlus::init().expect("Failed to initialize Display HAT Mini.");

        async {
            let mut display = unit.display.lock().await;

            display
                .backlight
                .transition_to(1., 32, Duration::from_secs(2))
                .await
                .expect("Failed to transition backlight to full power.");

            display.fill_white().expect("Failed to fill display white.");
            tokio::time::sleep(Duration::from_secs(1)).await;

            display.fill_red().expect("Failed to fill display red.");
            tokio::time::sleep(Duration::from_secs(1)).await;

            display.fill_green().expect("Failed to fill display green.");
            tokio::time::sleep(Duration::from_secs(1)).await;

            display.fill_blue().expect("Failed to fill display blue.");
            tokio::time::sleep(Duration::from_secs(1)).await;

            display
                .backlight
                .transition_to(0., 32, Duration::from_secs(2))
                .await
                .expect("Failed to transition backlight to dark.");
        }
        .await;
    }
}
