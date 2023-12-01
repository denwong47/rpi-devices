//! Move the entire image by the given amount of pixels.
//!

use super::TransitionFunction;
use crate::{foreign_types::*, func, SubImage};

/// Move the entire image by the given amount of pixels.
pub fn transverse<'a, COLOUR, T>(
    // These are all unsigned integers because they are used as offsets.
    // You cannot transverse from a negative position of an image.
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
) -> impl TransitionFunction<'a, COLOUR, T, T, SubImage<'a, T>>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR> + 'a,
{
    let delta_x = end_x as i32 - start_x as i32;
    let delta_y = end_y as i32 - start_y as i32;

    println!("delta_x: {}, delta_y: {}", delta_x, delta_y);

    // The second image is not used.
    move |from: &'a T,
          _: &'a T,
          step: u32,
          steps: u32,
          w: u16,
          h: u16|
          -> RPiResult<'a, SubImage<'a, T>> {
        let ratio = step as f32 / steps as f32;
        let (dx, dy) = (
            (ratio * delta_x as f32) as i32 + start_x as i32,
            (ratio * delta_y as f32) as i32 + start_y as i32,
        );

        println!("step: {}, dx: {}, dy: {}", step, dx, dy);

        Ok(func::crop::crop_raw(from, dx, dy, w as u32, h as u32))
    }
}
