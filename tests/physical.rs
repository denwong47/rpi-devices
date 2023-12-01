#![allow(unused_imports)]

//! These tests are only useful if you have the physical board attached to your device.
//! They are ignored by default, but can be run using the feature that corresponds to the board.
//!

use std::ops::DerefMut;

use rpi_devices::{
    display_mipidsi::{func as img_func, *},
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

    #[tokio::test]
    async fn physical_press() {
        let gpio = func::init_gpio().unwrap();

        logger::debug(&format!(
            "\x1b[38;5;11mPress a button on the Pimoroni Display HAT Mini...\x1b[39m"
        ));
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

        for slice in rgbs.windows(2).into_iter() {
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

        logger::debug(&format!(
            "\x1b[38;5;11mTransitioning the backlight from OFF to 100%...\x1b[39m"
        ));
        backlight
            .transition_to(1.0, 32, Duration::from_secs(2))
            .await
            .expect("Failed to transition to 1.0");

        logger::debug(&format!(
            "\x1b[38;5;11mDisabling the backlight for 1s...\x1b[39m"
        ));
        backlight.disable().expect("Failed to disable");
        tokio::time::sleep(Duration::from_secs(1)).await;

        logger::debug(&format!(
            "\x1b[38;5;11mRe-enabling the backlight for 1s...\x1b[39m"
        ));
        backlight.enable().expect("Failed to enable");
        tokio::time::sleep(Duration::from_secs(1)).await;

        logger::debug(&format!(
            "\x1b[38;5;11mTransitioning the backlight from 100% to OFF...\x1b[39m"
        ));
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
            let mut display = unit.display.lock().await;
            let bytes_array = load_bytes().await.expect("Failed to load bytes.");
            let raws = load_raw::<pixelcolor::Rgb565, 320>(&bytes_array);

            display.fill_black().expect("Failed to fill display black.");

            const STEPS: u32 = 40;

            for (id, raw) in raws.into_iter().enumerate() {
                if id == 0 {
                    display
                        .draw_image(img_func::image_conversions::image_from_raw(&raw, 0, 0))
                        .expect("Failed to draw image.");

                    // For the first image, once we have loaded the image to the screen, we
                    // fade in.
                    display
                        .backlight
                        .transition_to(1., 32, Duration::from_secs(2))
                        .await
                        .expect("Failed to transition backlight to full power.");
                } else {
                    let sweeper = img_func::transitions::sweep(
                        40,
                        img_func::transitions::SweepDirection::FromLeft,
                    );

                    img_func::transitions::Transition::new_self(
                        &mut display.screen,
                        &raw,
                        sweeper,
                        STEPS,
                        Duration::from_secs(2),
                    )
                    .start()
                    .await
                    .expect("Failed to transition image.");
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            }

            display
                .backlight
                .transition_to(0., 32, Duration::from_secs(2))
                .await
                .expect("Failed to transition backlight to dark.");
            display.fill_black().expect("Failed to fill display black.");
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
            display.draw_image(image).expect("Failed to draw image.");

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
