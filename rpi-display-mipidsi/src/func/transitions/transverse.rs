//! Move the entire image by the given amount of pixels.
//!

use super::traits::DrawTransition;
use crate::{foreign_types::*, func};

/// Move the entire image by the given amount of pixels.
pub fn transverse<'a, COLOUR, T, DT>(
    // These are all unsigned integers because they are used as offsets.
    // You cannot transverse from a negative position of an image.
    steps: u32,
    start_x: u32,
    start_y: u32,
    end_x: u32,
    end_y: u32,
) -> impl DrawTransition<'a, COLOUR, T, T, DT>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR> + 'a,
    DT: DrawTarget<Color = COLOUR> + 'a,
    DT::Error: Into<RPiError<'a>>,
{
    let delta_x = end_x as i32 - start_x as i32;
    let delta_y = end_y as i32 - start_y as i32;

    // The second image is not used.
    move |target: &mut DT, from: &'a T, _: &'a T, step: u32| -> RPiResult<'a, ()> {
        let ratio = step as f32 / steps as f32;
        let (dx, dy) = (
            (ratio * delta_x as f32) as i32 + start_x as i32,
            (ratio * delta_y as f32) as i32 + start_y as i32,
        );

        #[cfg(feature = "debug")]
        logger::trace(&format!(
            "Transversing step: {}, dx: {}, dy: {}",
            step, dx, dy
        ));

        let size = from.size();

        func::crop::crop_raw(from, dx, dy, size.width, size.height)
            .draw(target)
            .into_rpi_result()
    }
}
