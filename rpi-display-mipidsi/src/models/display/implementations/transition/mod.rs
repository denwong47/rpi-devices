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
    /// Draw a already defined text box on the display.
    pub async fn transition_to<'a, 'e, T, F>(
        &'a mut self,
        target: &'e T,
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
        let mut handler = transitions::Transition::new(
            &mut self.screen,
            target,
            target,
            transition,
            steps,
            duration,
        );

        handler.start().await
    }
}
