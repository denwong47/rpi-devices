//! Implementations for conversions from other errors into [`RPiError`].
//!

#[cfg(doc)]
use crate::RPiError;

mod into_rpi_result;

#[cfg(feature = "display")]
mod display;

#[cfg(feature = "bmp")]
mod bmp;
