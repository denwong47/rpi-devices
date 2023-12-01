use crate::{foreign_types::*, LcdDisplay};

impl<DI, MODEL, RST, const W: u16, const H: u16> LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
{
    /// Clears the display.
    pub fn fill(&mut self, colour: MODEL::ColorFormat) -> RPiResult<()> {
        self.clear(colour).map_err(|_| RPiError::DisplayOutputError)
    }
}

macro_rules! expand_preset_colours {
    ($((
        $func:ident,
        $colour:path,
        $name:expr
    )),*$(,)?) => {
        $(
            #[doc = "Fill the display with "]
            #[doc = $name]
            #[doc = " colour."]
            pub fn $func(&mut self) -> RPiResult<()> {
                self.fill($colour)
            }
        )*
    };
}

impl<DI, MODEL, RST, const W: u16, const H: u16> LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
{
    expand_preset_colours!(
        (fill_black, MODEL::ColorFormat::BLACK, "black"),
        (fill_white, MODEL::ColorFormat::WHITE, "white"),
        (fill_red, MODEL::ColorFormat::RED, "red"),
        (fill_green, MODEL::ColorFormat::GREEN, "green"),
        (fill_blue, MODEL::ColorFormat::BLUE, "blue"),
        (fill_cyan, MODEL::ColorFormat::CYAN, "cyan"),
        (fill_magenta, MODEL::ColorFormat::MAGENTA, "magenta"),
        (fill_yellow, MODEL::ColorFormat::YELLOW, "yellow"),
    );
}
