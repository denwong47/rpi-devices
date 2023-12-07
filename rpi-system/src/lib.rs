//! Raspberry Pi System module.
//!
//! This is a thin wrapper around [`rppal::system`], re-exporting the most common
//! structs and types while providing additional helper functions.

use rpi_errors::*;
use std::sync::{Arc, OnceLock};

pub use rppal::system::{DeviceInfo, Error as SystemError, Model, SoC};

/// A static reference to the [`DeviceInfo`] struct for the current device.
static DEVICE_INFO: OnceLock<Result<Arc<DeviceInfo>, SystemError>> = OnceLock::new();

/// A static reference to the [`Model`] enum for the current device.
static MODEL: OnceLock<Result<Arc<Model>, SystemError>> = OnceLock::new();

/// A static reference to the [`SoC`] enum for the current device.
static SOC: OnceLock<Result<Arc<SoC>, SystemError>> = OnceLock::new();

/// Helper function to get a static reference to a [`OnceLock`].
fn get_static<'e, T, F>(
    data: &OnceLock<Result<Arc<T>, SystemError>>,
    getter: F,
) -> RPiResult<'e, Arc<T>>
where
    F: Fn() -> Result<T, SystemError>,
{
    data.get_or_init(|| getter().map(Arc::new))
        .as_ref()
        .map(Arc::clone)
        .map_err(|e| RPiError::System(e.to_string().into()))
}

/// Get the [`DeviceInfo`] struct for the current device.
pub fn device_info<'e>() -> RPiResult<'e, Arc<DeviceInfo>> {
    get_static(&DEVICE_INFO, rppal::system::DeviceInfo::new)
}

/// Get the [``] struct for the current device.
pub fn model<'e>() -> RPiResult<'e, Arc<Model>> {
    let device_info = device_info()?;
    get_static(&MODEL, || Ok(device_info.model()))
}

/// Get the [`SoC`] struct for the current device.
pub fn soc<'e>() -> RPiResult<'e, Arc<SoC>> {
    let device_info = device_info()?;
    get_static(&SOC, || Ok(device_info.soc()))
}

/// Not really tests; just a way to check the output of the functions.
#[cfg(test)]
mod printout {
    use super::*;

    #[test]
    fn test_device_info() {
        let device_info = device_info().unwrap();
        println!("Device Info: {:?}", device_info);
    }

    #[test]
    fn test_model() {
        let model = model().unwrap();
        println!("Model: {:?}", model);
    }

    #[test]
    fn test_soc() {
        let soc = soc().unwrap();
        println!("SoC: {:?}", soc);
    }
}
