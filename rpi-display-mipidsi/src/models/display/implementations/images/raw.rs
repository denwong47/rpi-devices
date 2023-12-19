//! Display an image that is already encoded in the same colour format as the display,
//! without any header or metadata information.
//!
use crate::{foreign_types::*, LcdDisplay};

impl<DI, MODEL, RST, const W: u16, const H: u16> LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
    MODEL::ColorFormat: From<<MODEL::ColorFormat as PixelColor>::Raw>,
{
    /// Draw an image on the display.
    ///
    /// The image must be encoded in the same colour format as the display; no
    /// header or metadata information is allowed.
    ///
    /// It will be drawn from the top left corner of the display, and any pixels
    /// that are outside the display area will be ignored.
    pub fn draw_image<'e, T>(&mut self, image: &Image<'_, T>) -> RPiResult<'e, ()>
    where
        T: ImageDrawable<Color = MODEL::ColorFormat>,
    {
        image.draw(&mut self.screen).into_rpi_result()
    }
}
