//! Foreign types used in the library.
//!
//! This module contains types that are not defined in this library, but are used
//! by it. This reduces code duplication between the different modules.
pub(crate) use display_interface::WriteOnlyDataCommand;
pub(crate) use display_interface_spi::SPIInterfaceNoCS;
pub(crate) use embedded_graphics::draw_target::DrawTarget;
pub(crate) use embedded_graphics::pixelcolor::RgbColor;
pub(crate) use mipidsi::{
    models::Model as DisplayModel, Builder as RawDisplayBuilder, ColorInversion,
    Display as RawDisplay, Orientation,
};
pub(crate) use rpi_errors::{IntoRPiResult, RPiError, RPiResult};
pub(crate) use rpi_gpio::{OutputPin, OutputPinType};
pub(crate) use rppal::hal::Delay;
pub(crate) use rppal::spi::Spi;
