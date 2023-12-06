//! Extension of the [`HardwareComponent`] traits from [`rpi_gpio`].
//!

use crate::foreign_types::*;
use rpi_gpio::traits::HardwareComponent;

/// A [`HardwareComponent`] with an [`LcdDisplay`].
pub trait DisplayComponent<const W: u16, const H: u16>: HardwareComponent {
    type COLOUR: PixelColor;
    type DI: WriteOnlyDataCommand;
    type MODEL: DisplayModel<ColorFormat = Self::COLOUR>;
    type RST: OutputPinType;
}
