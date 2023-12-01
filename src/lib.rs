pub mod boards;

/// A library for checking the parameters of the system.
pub mod system {
    pub use rpi_system::*;
}

/// A library for interacting with GPIO pins on a Raspberry Pi, with all polling done
/// asynchronously.
///
/// Supports [`Button`]s and [RGB LED]s.
///
/// This is currently written for the express purpose of using a
/// [Pimoroni Display HAT Mini] on a Pi Zero 2W; features are added as required.
///
/// # Note
///
/// Re-export of [`rpi_gpio`].
///
/// [Pimoroni Display HAT Mini]: https://pinout.xyz/pinout/display_hat_mini
pub mod gpio {
    pub use rpi_gpio::*;
}

/// Error classes for this whole crate.
///
/// # Note
///
/// Re-export of [`rpi_errors`].
pub mod errors {
    pub use rpi_errors::*;
}

/// A library for interacting with typical MIPI Display Serial Interface devices on
/// Raspberry PI GPIO Pins, commonly used for LCD displays.
pub mod display_mipidsi {
    /// Re-export of [`rpi_display_mipidsi`]
    pub use rpi_display_mipidsi::*;
}

/// Logger for this whole crate.
///
/// # Note
///
/// Re-export of [`rpi_logger`].
pub mod logger {
    pub use rpi_logger::*;
}
