use rpi_gpio::DisplayBacklight;

use crate::foreign_types::*;

#[allow(dead_code)]
pub struct LcdDisplay<DI, MODEL, RST, const W: u16, const H: u16>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
{
    _delay: Delay, // this is just to hold a reference; a mutable reference is needed for the SPI interface, and there is nothing we can do to this afterwards.

    pub backlight: DisplayBacklight,

    pub(crate) display: RawDisplay<DI, MODEL, RST>,
}

macro_rules! expand_preset_models {
    ($((
        $model:path,
        $name:literal
    )),*) => {
        $(
            impl<DI, RST, const W: u16, const H: u16> LcdDisplay<DI, $model, RST, W, H>
            where
                DI: WriteOnlyDataCommand,
                RST: OutputPinType,
            {
                #[doc = "Create a new "]
                #[doc = $name]
                #[doc = " display unit."]
                pub fn new<'e>(
                    di: DI,
                    rst: Option<RST>,
                    mut delay: Delay,
                    orientation: Orientation,
                    colour_inversion: ColorInversion,
                    backlight: DisplayBacklight,
                ) -> RPiResult<'e, Self> {
                    let display =
                        RawDisplayBuilder::with_model(di, $model)
                        // width and height are switched on purpose because of the orientation
                        .with_display_size(H, W)
                        .with_orientation(orientation)
                        .with_invert_colors(colour_inversion)
                        .init(&mut delay, rst)
                        .into_rpi_result()?
                    ;

                    Ok(Self {
                        _delay: delay,
                        backlight,
                        display,
                    })
                }
            }
        )*
    };
}

expand_preset_models!(
    (mipidsi::models::ST7735s, "ST7735s"),
    (crate::models::panels::ST7735<true>, "ST7735"),
    (crate::models::panels::ST7735<false>, "ST7735"),
    (mipidsi::models::ST7789, "ST7789")
);

/// Convenience type for a SPI interface with a GPIO pin for reset.
pub type SpiLcdDisplay<MODEL, const W: u16, const H: u16> =
    LcdDisplay<SPIInterfaceNoCS<Spi, OutputPin>, MODEL, OutputPin, W, H>;

/// Convenience type for a ST7735S display with a SPI interface and a GPIO pin for reset.
pub type LcdST7735s<const W: u16, const H: u16> = SpiLcdDisplay<mipidsi::models::ST7735s, W, H>;

/// Convenience type for a ST7735S display with a SPI interface and a GPIO pin for reset.
pub type LcdST7735<const W: u16, const H: u16, const INVERT: bool = true> =
    SpiLcdDisplay<crate::models::panels::ST7735<INVERT>, W, H>;

/// Convenience type for a ST7789 display with a SPI interface and a GPIO pin for reset.
pub type LcdST7789<const W: u16, const H: u16> = SpiLcdDisplay<mipidsi::models::ST7789, W, H>;
