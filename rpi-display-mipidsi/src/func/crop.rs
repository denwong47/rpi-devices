//! Crop an image.
//!

use crate::foreign_types::*;
use crate::{func, pixelcolor::PixelColor, primitives, ImageDrawableExt, Point, Size, SubImage};

/// Create an [`SubImage`] from an [`ImageRaw`], at the given position and size.
pub fn crop_raw<COLOUR, T>(raw: &T, x: i32, y: i32, w: u32, h: u32) -> SubImage<'_, T>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR>,
{
    ImageDrawableExt::sub_image(
        raw,
        &primitives::Rectangle::new(Point::new(x, y), Size::new(w, h)),
    )
}

/// Create an [`SubImage`] from an [`ImageRaw`], within the given bounding box.
pub fn crop_raw_to<COLOUR, T>(raw: &T, x1: i32, y1: i32, x2: i32, y2: i32) -> SubImage<'_, T>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR>,
{
    // Make sure the bounding box is valid.
    let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
    let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

    crop_raw(raw, x1, y1, (x2 - x1) as u32, (y2 - y1) as u32)
}

/// Create an [`SubImage`] from an [`ImageRaw`], cropping away the left and
/// right part of the image from the given X coordinate.
pub fn crop_horizontal<COLOUR, T>(raw: &T, at: i32, width: u32) -> SubImage<'_, T>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR>,
{
    let Size {
        width: w,
        height: h,
    } = raw.size();

    crop_raw(
        raw,
        at,
        0,
        (w as i32 - at).min(width as i32).max(0) as u32,
        h,
    )
}

/// Create an [`SubImage`] from an [`ImageRaw`], cropping away the top and
/// bottom part of the image from the given Y coordinate.
pub fn crop_vertical<COLOUR, T>(raw: &T, at: i32, height: u32) -> SubImage<'_, T>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR>,
{
    let Size {
        width: w,
        height: h,
    } = raw.size();

    crop_raw(
        raw,
        0,
        at,
        w,
        (h as i32 - at).min(height as i32).max(0) as u32,
    )
}

/// Create an [`Image`] from an [`ImageDrawable`] by cropping it, and offsetting
/// it to keep it in the same position as the original image.
pub fn draw_cropped_in_place<'e, COLOUR, T, DT>(
    target: &mut DT,
    raw: &T,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> RPiResult<'e, ()>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR>,
    DT: DrawTarget<Color = COLOUR>,
    DT::Error: Into<RPiError<'e>>,
{
    let cropped = crop_raw_to(raw, x1, y1, x2, y2);

    func::image_conversions::image_from_raw(&cropped, x1, y1)
        .draw(target)
        .into_rpi_result()
}
