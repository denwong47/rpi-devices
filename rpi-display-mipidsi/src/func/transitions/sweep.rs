//! Sweep an image to transition over to another image.

use super::traits::DrawTransition;
use crate::{foreign_types::*, func};

/// The direction to sweep the image.
pub enum SweepDirection {
    FromLeft,
    FromRight,
    FromTop,
    FromBottom,
    InsideOut,
}

impl SweepDirection {
    pub fn direction(&self) -> &'static str {
        match self {
            Self::FromLeft => "from left to right",
            Self::FromRight => "from right to left",
            Self::FromTop => "from top to bottom",
            Self::FromBottom => "from bottom to top",
            Self::InsideOut => "from inside to outside",
        }
    }
}

/// Move the entire image by the given amount of pixels.
pub fn sweep<'a, COLOUR, T, DT>(
    // These are all unsigned integers because they are used as offsets.
    // You cannot transverse from a negative position of an image.
    steps: u32,
    direction: SweepDirection,
) -> impl DrawTransition<'a, COLOUR, T, T, DT>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR> + 'a,
    DT: DrawTarget<Color = COLOUR> + 'a,
    DT::Error: Into<RPiError<'a>>,
{
    // The second image is not used.
    move |target: &mut DT, from: &'a T, _: &'a T, step: u32| -> RPiResult<'a, ()> {
        let ratio = ((step + 1) as f32 / steps as f32).min(1.);
        let size = target.bounding_box().size;

        let (offset, size) = match direction {
            SweepDirection::FromLeft => (
                (0, 0),
                ((ratio * size.width as f32) as i32, size.height as i32),
            ),
            SweepDirection::FromRight => {
                let w = (ratio * size.width as f32) as i32;
                let dx = size.width as i32 - w;
                ((dx, 0), (w, size.height as i32))
            }
            SweepDirection::FromTop => (
                (0, 0),
                (size.width as i32, (ratio * size.height as f32) as i32),
            ),
            SweepDirection::FromBottom => {
                let h = (ratio * size.height as f32) as i32;
                let dy = size.height as i32 - h;
                ((0, dy), (size.width as i32, h))
            }
            SweepDirection::InsideOut => {
                let w = (ratio * size.width as f32) as i32;
                let h = (ratio * size.height as f32) as i32;
                let dx = (size.width as i32 - w) / 2;
                let dy = (size.height as i32 - h) / 2;
                ((dx, dy), (w, h))
            }
        };

        #[cfg(feature = "debug")]
        logger::debug(&format!(
            "Sweeping {} step: {}, offset: {:?}, size: {:?}",
            direction.direction(),
            step,
            offset,
            size
        ));

        func::crop::draw_cropped_in_place(
            target,
            from,
            offset.0,
            offset.1,
            offset.0 + size.0,
            offset.1 + size.1,
        )
        .into_rpi_result()
    }
}
