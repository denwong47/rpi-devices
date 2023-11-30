//! Foreign types used in the library.
//!
//! This module contains types that are not defined in this library, but are used
//! by it. This reduces code duplication between the different modules.
pub(crate) use display_interface::{DataFormat, WriteOnlyDataCommand};
pub(crate) use display_interface_spi::SPIInterfaceNoCS;
pub(crate) use embedded_graphics::draw_target::DrawTarget;
pub(crate) use embedded_graphics::pixelcolor::RgbColor;
pub(crate) use embedded_graphics::prelude::IntoStorage;
pub(crate) use embedded_hal::blocking::delay::DelayUs;
pub(crate) use mipidsi::{
    dcs::{self, Dcs},
    error::InitError as DisplayInitError,
    models::Model as DisplayModel,
    Builder as RawDisplayBuilder, ColorInversion, Display as RawDisplay, Error as MipidsiError,
    ModelOptions as DisplayModelOptions, Orientation,
};
pub(crate) use rpi_errors::{IntoRPiResult, RPiError, RPiResult};
pub(crate) use rpi_gpio::{OutputPin, OutputPinType};
pub(crate) use rppal::hal::Delay;
pub(crate) use rppal::spi::Spi;
