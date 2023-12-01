//! Conversions between [`embedded_graphics`] image formats.

use crate::foreign_types::*;

/// Create an [`Image`] from an [`ImageRaw`], at the given position.
pub fn image_from_raw<'a, COLOUR, T>(raw: &'a T, x: i32, y: i32) -> Image<'a, T>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR>,
{
    Image::new(raw, Point::new(x, y))
}

/// Create an [`ImageRaw`] from a slice of [`u8`]s, rendered using the given
/// width.
pub fn raw_from_bytes<'a, COLOUR>(bytes: &'a [u8], width: u32) -> ImageRaw<'a, COLOUR>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
{
    ImageRaw::new(bytes, width)
}
