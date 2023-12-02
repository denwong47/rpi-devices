//! Default [`CharacterStyle`]s
//!
//!
use super::reexports::*;
use crate::foreign_types::*;

/// A simple collection of [`CharacterStyle`]s for commonly used text heights.
pub struct DefaultStyle<const H: u8>;

/// A marker trait for [`CharacterStyle`]s that can be used as a default.
pub trait ValidStyle {
    fn default_style<'s, COLOUR>(colour: COLOUR) -> MonoTextStyle<'s, COLOUR>
    where
        COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>;
}

macro_rules! expand_font_heights {
    ($((
        $height:literal, $font:ident
    )),*$(,)?) => {
        $(
            impl ValidStyle for DefaultStyle<$height>
            {
                fn default_style<'s, COLOUR>(colour: COLOUR) -> MonoTextStyle<'s, COLOUR>
                where
                    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
                {
                    MonoTextStyleBuilder::new()
                        .font(&$font)
                        .text_color(colour)
                        .build()
                }
            }
        )*
    };
}

expand_font_heights!(
    (6, FONT_4X6),
    (7, FONT_5X7),
    (8, FONT_5X8),
    (9, FONT_6X9),
    (10, FONT_6X10),
    (12, FONT_6X12),
    (13, FONT_7X13),
    (15, FONT_9X15),
    (18, FONT_9X18),
    (20, FONT_10X20),
);
