//! Add text to the display.
//!
use super::{defaults::*, reexports::*};
use crate::{foreign_types::*, LcdDisplay};

impl<DI, MODEL, RST, const W: u16, const H: u16> LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
    MODEL::ColorFormat: From<<MODEL::ColorFormat as PixelColor>::Raw>,
{
    /// Draw a already defined text box on the display.
    pub fn draw_textbox<'e, 't, S, M>(
        &mut self,
        textbox: &TextBox<'t, S, M>,
    ) -> RPiResult<'e, String>
    where
        MODEL::ColorFormat: Default,
        S: TextRenderer<Color = MODEL::ColorFormat> + CharacterStyle<Color = MODEL::ColorFormat>,
        M: plugin::PluginMarker<'t, MODEL::ColorFormat>,
    {
        textbox
            .draw(&mut self.screen)
            .into_rpi_result()
            .and_then(|string| {
                #[cfg(feature = "debug")]
                if string.len() > 0 {
                    logger::trace(&format!("Skipped string: {}", string.trim()));
                }

                Ok(string.trim().to_owned())
            })
    }

    /// Draw a piece of text on the display.
    pub fn draw_raw_text<'e, 't, S, M>(
        &mut self,
        text: &'t str,
        character_style: S,
        textbox_style: TextBoxStyle,
        position: Option<Point>,
        size: Option<Size>,
        plugin: Option<M>,
    ) -> RPiResult<'e, String>
    where
        MODEL::ColorFormat: Default,
        S: TextRenderer<Color = MODEL::ColorFormat> + CharacterStyle<Color = MODEL::ColorFormat>,
        M: plugin::PluginMarker<'t, MODEL::ColorFormat>,
    {
        let position = position.unwrap_or_else(|| Point::zero());
        let screen_size = self.screen.size();
        let bounding_box = primitives::Rectangle::new(
            position,
            size.unwrap_or_else(|| {
                Size::new(
                    (screen_size.width as i32 - position.x) as u32,
                    (screen_size.height as i32 - position.y) as u32,
                )
            }),
        );

        let textbox =
            TextBox::with_textbox_style(text, bounding_box, character_style, textbox_style);

        if let Some(plugin) = plugin {
            self.draw_textbox(&textbox.add_plugin(plugin))
        } else {
            self.draw_textbox(&textbox)
        }
    }

    /// Draw a piece of ANSI text on the display.
    pub fn draw_ansi_text<'e, 't, S>(
        &mut self,
        text: &'t str,
        character_style: S,
        textbox_style: TextBoxStyle,
        position: Option<Point>,
        size: Option<Size>,
    ) -> RPiResult<'e, String>
    where
        MODEL::ColorFormat: Default + From<pixelcolor::Rgb888>,
        S: TextRenderer<Color = MODEL::ColorFormat> + CharacterStyle<Color = MODEL::ColorFormat>,
    {
        self.draw_raw_text(
            text,
            character_style,
            textbox_style,
            position,
            size,
            Some(plugin::ansi::Ansi::new()),
        )
    }

    /// Draw a piece of ANSI capable text using all the default settings.
    pub fn draw_text<'e, 't, const FS: u8>(
        &mut self,
        text: &'t str,
        colour: MODEL::ColorFormat,
        position: Option<Point>,
        size: Option<Size>,
    ) -> RPiResult<'e, String>
    where
        DefaultStyle<FS>: ValidStyle,
        MODEL::ColorFormat: Default + From<pixelcolor::Rgb888>,
    {
        self.draw_ansi_text(
            text,
            DefaultStyle::<FS>::default_style(colour),
            TextBoxStyle::default(),
            position,
            size,
        )
    }
}
