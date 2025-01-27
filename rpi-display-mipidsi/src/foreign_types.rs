//! Foreign types used in the library.
//!
//! This module contains types that are not defined in this library, but are used
//! by it. This reduces code duplication between the different modules.
pub(crate) use display_interface::{DataFormat, WriteOnlyDataCommand};
pub(crate) use display_interface_spi::SPIInterfaceNoCS;
#[cfg(feature = "text")]
pub(crate) use embedded_graphics::geometry::OriginDimensions;
pub(crate) use embedded_graphics::geometry::{Dimensions, Size};
pub(crate) use embedded_graphics::image::{Image, ImageRaw};
pub(crate) use embedded_graphics::pixelcolor;
pub(crate) use embedded_graphics::prelude::{
    DrawTarget, Drawable, ImageDrawable, IntoStorage, PixelColor, Point, RgbColor,
};
pub(crate) use embedded_graphics::primitives::{self, Primitive, PrimitiveStyle};
pub(crate) use embedded_hal::blocking::delay::DelayUs; // Watch out for this guy - v1.0.0 inbound
pub(crate) use mipidsi::{
    dcs::{self, Dcs},
    error::InitError as DisplayInitError,
    models::Model as DisplayModel,
    Builder as RawDisplayBuilder, ColorInversion, Display as RawDisplay, Error as MipidsiError,
    ModelOptions as DisplayModelOptions, Orientation, TearingEffect,
};
pub(crate) use rpi_errors::{IntoRPiResult, RPiError, RPiResult};
pub(crate) use rpi_gpio::{OutputPin, OutputPinType};
pub(crate) use rppal::hal::Delay;
pub(crate) use rppal::spi::Spi;

#[cfg(feature = "debug")]
pub(crate) use rpi_logger as logger;

#[cfg(any(feature = "text", feature = "bmp"))]
pub use tinybmp::{Bmp, RawBmp};
