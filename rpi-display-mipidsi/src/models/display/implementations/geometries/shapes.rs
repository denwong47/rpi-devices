//! Implementations of shape drawing methods.
//!

use crate::{foreign_types::*, LcdDisplay};

impl<DI, MODEL, RST, const W: u16, const H: u16> LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
    MODEL::ColorFormat: From<<MODEL::ColorFormat as PixelColor>::Raw>,
{
    /// Draw a rectangle on the display.
    pub fn draw_rect<'e>(
        &mut self,
        position: Point,
        size: Size,
        colour: MODEL::ColorFormat,
    ) -> RPiResult<'e, ()> {
        primitives::Rectangle::new(position, size)
            .into_styled(PrimitiveStyle::with_fill(colour))
            .draw(&mut self.screen)
            .into_rpi_result()
    }
}
