//! Conversions between [`embedded_graphics`] image formats.

use crate::foreign_types::*;

/// Create an [`Image`] from an [`ImageRaw`], at the given position.
pub fn image_from_raw<COLOUR, T>(raw: &T, x: i32, y: i32) -> Image<'_, T>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR>,
{
    Image::new(raw, Point::new(x, y))
}

/// Create an [`ImageRaw`] from a slice of [`u8`]s, rendered using the given
/// width.
pub fn raw_from_bytes<COLOUR>(bytes: &[u8], width: u32) -> ImageRaw<'_, COLOUR>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
{
    ImageRaw::new(bytes, width)
}
