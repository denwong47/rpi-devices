//! User interface implementation.
//!
use crate::{foreign_types::*, traits::UserInterface, LcdDisplay};
use std::time::Duration;

impl<DI, MODEL, RST, const W: u16, const H: u16> LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
    MODEL::ColorFormat: From<<MODEL::ColorFormat as PixelColor>::Raw>,
{
    /// Draw a line on the display.
    pub async fn draw_interface<'e, R>(
        &mut self,
        interface: &mut dyn UserInterface<DI, MODEL, RST, W, H, Return = R>,
    ) -> RPiResult<'e, R> {
        interface.execute(self).await
    }

    /// Fade the display to draw the interface.
    pub async fn fade_to_interface<'e, R>(
        &mut self,
        interface: &mut dyn UserInterface<DI, MODEL, RST, W, H, Return = R>,
        steps: u32,
        duration: Duration,
    ) -> RPiResult<'e, R> {
        let restore_value = self.backlight.value();
        self.backlight
            .transition_to(0., steps / 2, duration / 2)
            .await?;
        let result = self.draw_interface(interface).await;
        self.backlight
            .transition_to(restore_value, steps / 2, duration / 2)
            .await?;

        result
    }
}
