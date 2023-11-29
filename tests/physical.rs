//! These tests are only useful if you have the physical board attached to your device.
//! They are ignored by default, but can be run using the feature that corresponds to the board.

#[cfg(feature = "pimoroni-display-hat-mini")]
mod pimoroni_display_hat_mini {
    use std::time::Duration;

    use rpi_devices::{
        boards::PimoroniDisplayHATMini,
        gpio::{func, Button, DisplayBacklight, RgbLed},
    };

    use serial_test::serial;

    #[tokio::test]
    async fn physical_press() {
        let gpio = func::init_gpio().unwrap();

        println!("\x1b[38;5;11mPress a button on the Pimoroni Display HAT Mini...\x1b[39m");
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
                            println!("Pressed {}", stringify!($name));
                        } else {
                            println!("Timeout {} or error", stringify!($name));
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

        println!("\x1b[38;5;11mTransitioning the backlight from OFF to 100%...\x1b[39m");
        backlight
            .transition_to(1.0, 32, Duration::from_secs(2))
            .await
            .expect("Failed to transition to 1.0");

        println!("\x1b[38;5;11mDisabling the backlight for 1s...\x1b[39m");
        backlight.disable().expect("Failed to disable");
        tokio::time::sleep(Duration::from_secs(1)).await;

        println!("\x1b[38;5;11mRe-enabling the backlight for 1s...\x1b[39m");
        backlight.enable().expect("Failed to enable");
        tokio::time::sleep(Duration::from_secs(1)).await;

        println!("\x1b[38;5;11mTransitioning the backlight from 100% to OFF...\x1b[39m");
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
}
