//! A library for interacting with GPIO pins on a Raspberry Pi, with all polling done
//! asynchronously.
//!
//! Supports [`Button`]s and [RGB LED]s.
//!
//! This is currently written for the express purpose of using a
//! [Pimoroni Display HAT Mini] on a Pi Zero 2W; features are added as required.
//!
//! Not designed for general use.
//!
//! [RGB LED]: `RgbLed`
//! [Pimoroni Display HAT Mini]: https://shop.pimoroni.com/products/display-hat-mini?variant=39496084717651

pub mod config;

pub mod func;

mod models;
pub use models::*;

pub use embedded_hal::digital::v2::OutputPin as OutputPinType;

pub use rppal::gpio::{InputPin, OutputPin};
