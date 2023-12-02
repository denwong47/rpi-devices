//! Re-export of the `embedded_text` crate.
//!

pub use embedded_graphics::{
    mono_font::{ascii::*, MonoFont, MonoTextStyle, MonoTextStyleBuilder},
    text::renderer::{CharacterStyle, TextRenderer},
};

pub use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    plugin,
    style::{HeightMode, TextBoxStyle, TextBoxStyleBuilder},
    TextBox,
};
