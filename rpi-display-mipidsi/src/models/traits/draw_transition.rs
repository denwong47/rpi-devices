//! The base transition function.
//!

use crate::foreign_types::*;

/// A marker trait for functions capable of drawing a frame directly to a
/// [`DrawTarget`].
pub trait DrawTransition<'a, COLOUR, T1, T2, DT>
where
    DT: DrawTarget<Color = COLOUR>,
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T1: ImageDrawable<Color = COLOUR> + 'a,
    T2: ImageDrawable<Color = COLOUR> + 'a,
{
    fn draw_frame(&self, target: &mut DT, from: &'a T1, to: &'a T2, step: u32)
        -> RPiResult<'a, ()>;
}

impl<'a, COLOUR, T1, T2, DT, F> DrawTransition<'a, COLOUR, T1, T2, DT> for F
where
    DT: DrawTarget<Color = COLOUR> + 'a,
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T1: ImageDrawable<Color = COLOUR> + 'a,
    T2: ImageDrawable<Color = COLOUR> + 'a,
    F: Fn(&mut DT, &'a T1, &'a T2, u32) -> RPiResult<'a, ()>,
{
    /// Calculate the frame for the given step.
    fn draw_frame(
        &self,
        target: &mut DT,
        from: &'a T1,
        to: &'a T2,
        step: u32,
    ) -> RPiResult<'a, ()> {
        self(target, from, to, step)
    }
}
