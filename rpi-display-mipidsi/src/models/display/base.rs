//! [`LcdDisplay`] type and instantiation methods, along with common models.

use rpi_gpio::DisplayBacklight;

// #[cfg(feature = "debug")]
// use std::time::Instant;

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

    pub screen: RawDisplay<DI, MODEL, RST>,
}

impl<DI, MODEL, RST, const W: u16, const H: u16> Dimensions for LcdDisplay<DI, MODEL, RST, W, H>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
{
    /// Get the dimension of the display.
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        self.screen.bounding_box()
    }
}

// // SIGNIFICANT PERFORMANCE DEGRADATION - just use `screen` directly
// impl<DI, MODEL, RST, const W: u16, const H: u16> DrawTarget for LcdDisplay<DI, MODEL, RST, W, H>
// where
//     DI: WriteOnlyDataCommand,
//     MODEL: DisplayModel,
//     RST: OutputPinType,
// {
//     type Color = MODEL::ColorFormat;
//     type Error = DisplayError;

//     /// Draw all pixels from an iterator onto the screen.
//     ///
//     /// # Note
//     ///
//     /// It appears that the implementation of a private trait of `DrawBatch` has a
//     /// significant effect on the performance of this function; however we can't
//     /// implement it on [`LcdDisplay`] because the trait is not exposed.
//     ///
//     /// Therefore using [`LcdDisplay`] directly as a [`DrawTarget`] is not recommended;
//     /// simply use the publicly accessible [`LcdDisplay::display`] field instead.
//     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
//     where
//         I: IntoIterator<Item = embedded_graphics::prelude::Pixel<Self::Color>>,
//     {
//         if cfg!(feature = "debug") {
//             #[cfg(feature = "debug")]
//             let start_time = Instant::now();

//             let result = self.screen.draw_iter(pixels);

//             #[cfg(feature = "debug")]
//             logger::trace(&format!(
//                 "Drawing to LcdDisplay took {}ms.",
//                 start_time.elapsed().as_millis()
//             ));

//             result
//         } else {
//             self.screen.draw_iter(pixels)
//         }
//     }
// }

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
                    tearing_effect: TearingEffect,
                    backlight: DisplayBacklight,
                ) -> RPiResult<'e, Self> {
                    #[cfg(feature = "debug")]
                    logger::info(&format!("Creating new {}x{} {} LcdDisplay...", W, H, $name));

                    let mut screen =
                        RawDisplayBuilder::with_model(di, $model)
                        // width and height are switched on purpose because of the orientation
                        .with_display_size(H, W)
                        .with_orientation(orientation)
                        .with_invert_colors(colour_inversion)
                        .init(&mut delay, rst)
                        .into_rpi_result()?
                    ;

                    if tearing_effect != TearingEffect::Off {
                        screen.set_tearing_effect(tearing_effect)
                            .into_rpi_result()?;
                    }

                    Ok(Self {
                        _delay: delay,
                        backlight,
                        screen,
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
