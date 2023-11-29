//! An [`Iterator`] that yields [`Rgb`] values in a transition between two [`Rgb`] values.
//!

use embedded_graphics::pixelcolor::RgbColor;

use super::{FromTupleRGB, RgbBetween};

/// An [`Iterator`] that yields [`Rgb`] values in a transition between two [`Rgb`] values.
pub struct RgbTransition<'r, RGB>
where
    RGB: RgbBetween<RGB> + RgbColor + FromTupleRGB,
{
    start: &'r RGB,
    end: &'r RGB,
    steps: u32,
    current_step: u32,
}

impl<'r, RGB> RgbTransition<'r, RGB>
where
    RGB: RgbBetween<RGB> + RgbColor + FromTupleRGB,
{
    /// Create a new [`RgbTransition`] between two colors.
    pub fn new(start: &'r RGB, end: &'r RGB, steps: u32) -> Self {
        Self {
            start,
            end,
            steps: steps.max(1),
            current_step: 0,
        }
    }
}

impl<'r, RGB> Iterator for RgbTransition<'r, RGB>
where
    RGB: RgbBetween<RGB> + RgbColor + FromTupleRGB,
{
    type Item = RGB;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_step >= self.steps {
            return None;
        } else if self.current_step == 0 {
            self.current_step += 1;
            // Save some calculations.
            return Some(*self.start);
        } else if self.current_step == self.steps - 1 {
            self.current_step += 1;
            // Save some calculations.
            return Some(*self.end);
        }

        let factor = self.current_step as f32 / self.steps as f32;
        self.current_step += 1;

        Some(self.start.rgb_between(self.end, factor))
    }
}

/// Marker trait for [`RgbColor`]s that can be used in an [`RgbTransition`].
pub trait CanRgbTransition<RGB>
where
    RGB: RgbBetween<RGB> + RgbColor + FromTupleRGB,
{
    /// Create a new [`RgbTransition`] between two colors.
    fn transition_to<'r>(&'r self, end: &'r RGB, steps: u32) -> RgbTransition<'r, RGB>;
}

impl<RGB> CanRgbTransition<RGB> for RGB
where
    RGB: RgbBetween<RGB> + RgbColor + FromTupleRGB,
{
    /// Create a new [`RgbTransition`] between two colors.
    fn transition_to<'r>(&'r self, end: &'r RGB, steps: u32) -> RgbTransition<'r, RGB> {
        RgbTransition::new(self, end, steps)
    }
}
