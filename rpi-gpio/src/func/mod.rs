//! Helper functions.
//!

use rppal::gpio::Gpio;
pub mod termination;

/// Initialize GPIO interface.
pub fn init_gpio() -> Result<Gpio, rppal::gpio::Error> {
    Gpio::new()
}
