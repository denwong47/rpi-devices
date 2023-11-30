//! Conversions between [`embedded_graphics`] image formats.

use crate::foreign_types::*;

/// Create an [`Image`] from an [`ImageRaw`], at the given position.
pub fn image_from_raw<'a, COLOUR>(
    raw: &'a ImageRaw<'a, COLOUR>,
    x: i32,
    y: i32,
) -> Image<'a, ImageRaw<'a, COLOUR>>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    for<'i> ImageRaw<'i, COLOUR>: ImageDrawable,
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
