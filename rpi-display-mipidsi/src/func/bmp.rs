//! Bitmap image functions.
//!

use crate::foreign_types::*;
use tinybmp::Bmp;

/// Load a bitmap image from a bytes array.
///
/// # Note
///
/// Only [`Rgb555`], [`Rgb565`], and [`Rgb888`] are supported.
///
/// [`Rgb555`]: pixelcolor::Rgb555
/// [`Rgb565`]: pixelcolor::Rgb565
/// [`Rgb888`]: pixelcolor::Rgb888
#[allow(dead_code)]
pub fn bmp_from_bytes<'e, COLOUR>(bytes: &[u8]) -> RPiResult<'e, Bmp<COLOUR>>
where
    COLOUR:
        PixelColor + From<pixelcolor::Rgb555> + From<pixelcolor::Rgb565> + From<pixelcolor::Rgb888>,
{
    Bmp::from_slice(bytes).into_rpi_result()
}
