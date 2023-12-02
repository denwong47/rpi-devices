//! Implementations of the `Transition` struct.
//!

use crate::{foreign_types::*, func::transitions, traits::DrawTransition, LcdDisplay};
use std::time::Duration;

impl<DI, MODEL, RST, const W: u16, const H: u16> LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
    MODEL::ColorFormat: From<<MODEL::ColorFormat as PixelColor>::Raw>,
{
    /// Transition from one image to another using the supplied
    /// [`DrawTransition`], and the given steps and duration.
    pub async fn draw_transition<'a, 'e, T1, T2, F>(
        &'a mut self,
        from: &'e T1,
        to: &'e T2,
        transition: F,
        steps: u32,
        duration: Duration,
    ) -> RPiResult<'e, ()>
    where
        'a: 'e,
        MODEL::ColorFormat: Default,
        T1: ImageDrawable<Color = MODEL::ColorFormat> + 'e,
        T2: ImageDrawable<Color = MODEL::ColorFormat> + 'e,
        F: DrawTransition<'e, MODEL::ColorFormat, T1, T2, mipidsi::Display<DI, MODEL, RST>>,
    {
        let mut handler =
            transitions::Transition::new(&mut self.screen, from, to, transition, steps, duration);

        handler.start().await
    }

    /// Transition to a new image using the supplied [`DrawTransition`], and the
    /// given steps and duration.
    pub async fn draw_transition_to<'a, 'e, T, F>(
        &'a mut self,
        frame: &'e T,
        transition: F,
        steps: u32,
        duration: Duration,
    ) -> RPiResult<'e, ()>
    where
        'a: 'e,
        MODEL::ColorFormat: Default,
        T: ImageDrawable<Color = MODEL::ColorFormat> + 'e,
        F: DrawTransition<'e, MODEL::ColorFormat, T, T, mipidsi::Display<DI, MODEL, RST>>,
    {
        self.draw_transition(frame, frame, transition, steps, duration)
            .await
    }
}
