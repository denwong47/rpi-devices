//! Implementations of line drawing methods.
//!

use crate::{foreign_types::*, LcdDisplay};

impl<DI, MODEL, RST, const W: u16, const H: u16> LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
    MODEL::ColorFormat: From<<MODEL::ColorFormat as PixelColor>::Raw>,
{
    /// Draw a line on the display.
    pub fn draw_line<'e>(
        &mut self,
        from: Point,
        to: Point,
        colour: MODEL::ColorFormat,
        stroke: u32,
    ) -> RPiResult<'e, ()> {
        primitives::Line::new(from, to)
            .into_styled(PrimitiveStyle::with_stroke(colour, stroke))
            .draw(&mut self.screen)
            .into_rpi_result()
    }

    /// Draw a line on the display from a given point to another given the offset.
    pub fn draw_line_from<'e>(
        &mut self,
        from: Point,
        dx: i32,
        dy: i32,
        colour: MODEL::ColorFormat,
        stroke: u32,
    ) -> RPiResult<'e, ()> {
        let to = Point::new(from.x + dx, from.y + dy);

        self.draw_line(from, to, colour, stroke)
    }

    /// Draw a horizontal line on the display.
    pub fn draw_horizontal_line<'e>(
        &mut self,
        y: i32,
        colour: MODEL::ColorFormat,
        stroke: u32,
    ) -> RPiResult<'e, ()> {
        self.draw_line(Point::new(0, y), Point::new(W as i32, y), colour, stroke)
    }

    // Draw a vertical line on the display.
    pub fn draw_vertical_line<'e>(
        &mut self,
        x: i32,
        colour: MODEL::ColorFormat,
        stroke: u32,
    ) -> RPiResult<'e, ()> {
        self.draw_line(Point::new(x, 0), Point::new(x, H as i32), colour, stroke)
    }
}
