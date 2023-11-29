use std::time::Duration;

use async_trait::async_trait;
use rpi_devices::{boards::PimoroniDisplayHATMini, errors::RPiResult, gpio::Button};

#[async_trait]
pub trait ColourOnPress {
    async fn run<'e>(&mut self) -> RPiResult<'e, ()>;
}

/// Transition the LED to the specified colour during button press,
/// stop and disable LED when the button is released.
async fn on_button_press<'e>(
    unit: &PimoroniDisplayHATMini,
    button: &Button,
    colour: (u8, u8, u8),
) -> RPiResult<'e, ()> {
    loop {
        if button.pressed(None).await? {
            // Transition the LED to the specified colour or wait for the button to be released.
            tokio::select! {
                transition_result = async {
                    let mut led = unit.led.lock().await;

                    led.transition_to(colour.0, colour.1, colour.2, 1<<8, Duration::from_secs(3)).await
                } => transition_result,
                release_result = async {
                    button.released(None).await
                } => release_result.and(Ok(())),
            }?
        } else {
            unreachable!("button was not pressed")
        }

        let mut led = unit.led.lock().await;
        led.disable()?;
    }
}

#[async_trait]
impl ColourOnPress for PimoroniDisplayHATMini {
    async fn run<'e>(&mut self) -> RPiResult<'e, ()> {
        macro_rules! press_action {
            ($((
                $colour:expr,
                $button:ident
            )),*) => {
                tokio::select! {
                    $(
                        $button = on_button_press(self, &self.$button, $colour) => $button,
                    )*
                }.and_then(|_| Ok(()))
            }
        }

        press_action!(
            ((255, 0, 0), button_a),
            ((0, 255, 0), button_b),
            ((0, 0, 255), button_x),
            ((255, 255, 255), button_y)
        )
    }
}
