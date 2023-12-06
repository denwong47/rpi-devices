//! Marker traits for GPIO related structs.
//!

/// A marker trait for structs that represents an off-the-shelf or widely available
/// physical hardware attachment, typically including a composite of GPIO/SPI/I2C
/// components.
pub trait HardwareComponent {}
