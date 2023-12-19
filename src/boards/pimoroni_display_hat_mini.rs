//! Pimoroni Display HAT Mini on a Raspberry Pi.
//!
//!
use async_trait::async_trait;

use crate::display_mipidsi::LcdST7789;
use crate::display_mipidsi::{
    traits::{BacklightComponent, DisplayComponent},
    ColorInversion, DisplaySPIInterfaceNoCS, Orientation, TearingEffect,
};
use crate::errors::{IntoRPiResult, RPiResult};
use crate::gpio::{func, traits::HardwareComponent, Button, DisplayBacklight, OutputPin, RgbLed};
use async_mutex::Mutex;
use rppal::{
    hal::Delay,
    spi::{Bus, Mode as SpiMode, SlaveSelect, Spi},
};
use std::marker::PhantomData;

/// Pimoroni Display HAT Mini on a Raspberry Pi.
pub struct PimoroniDisplayHATMini {
    // Prevents instantiation of this struct.
    _phantom: PhantomData<()>,
    pub button_a: Button,
    pub button_b: Button,
    pub button_x: Button,
    pub button_y: Button,
    pub led: Mutex<RgbLed>,

    pub display: Mutex<LcdST7789<320, 240>>,
}

impl PimoroniDisplayHATMini {
    pub const SPI_BUS: Bus = Bus::Spi0;
    pub const SPI_SLAVE: SlaveSelect = SlaveSelect::Ss1;
    pub const SPI_CLOCK_SPEED: u32 = 70_000_000;
    pub const SPI_MODE: SpiMode = SpiMode::Mode0;

    pub const SPI_MOSI: u8 = 10;
    pub const SPI_DC: u8 = 9;
    pub const SPI_SLCK: u8 = 11;
    pub const SPI_CS: u8 = 7;
    pub const DISPLAY_BACKLIGHT: u8 = 13;

    pub const DISPLAY_ORIENTATION: Orientation = Orientation::LandscapeInverted(true);
    pub const DISPLAY_COLOUR_INVERSION: ColorInversion = ColorInversion::Inverted;
    pub const DISPLAY_TEARING_EFFECT: TearingEffect = TearingEffect::HorizontalAndVertical;
    pub const DISPLAY_RESET: Option<u8> = None;

    pub const BUTTON_A: u8 = 5;
    pub const BUTTON_B: u8 = 6;
    pub const BUTTON_X: u8 = 16;
    pub const BUTTON_Y: u8 = 24;

    pub const LED_R: u8 = 17;
    pub const LED_G: u8 = 27;
    pub const LED_B: u8 = 22;

    /// Initialize the Pimoroni Display HAT Mini.
    ///
    /// # Note
    ///
    /// Creating two instances of this struct is undefined behaviour.
    pub fn init<'e>() -> RPiResult<'e, Self> {
        let gpio = func::init_gpio()?;

        let backlight = DisplayBacklight::new(&gpio, Self::DISPLAY_BACKLIGHT, 50.);

        // The SPI bus, without the Chip Select Line; supposingly this can be used by
        // more than one device.
        let spi = Spi::new(
            Self::SPI_BUS,
            Self::SPI_SLAVE,
            Self::SPI_CLOCK_SPEED,
            Self::SPI_MODE,
        )
        .into_rpi_result()?;

        let rst = RPiResult::Ok(Self::DISPLAY_RESET)
            .and_then(|pin_opt| {
                if let Some(pin) = pin_opt {
                    Ok(Some(gpio.get(pin)?.into_output()))
                } else {
                    Ok(None)
                }
            })
            .into_rpi_result()?;

        // The Chip Select Line.
        let dc = gpio.get(Self::SPI_DC).into_rpi_result()?.into_output();

        let di = DisplaySPIInterfaceNoCS::new(spi, dc);

        Ok(Self {
            _phantom: PhantomData,
            button_a: Button::new(&gpio, Self::BUTTON_A),
            button_b: Button::new(&gpio, Self::BUTTON_B),
            button_x: Button::new(&gpio, Self::BUTTON_X),
            button_y: Button::new(&gpio, Self::BUTTON_Y),

            led: RgbLed::new(&gpio, Self::LED_R, Self::LED_G, Self::LED_B, 50.).into(),

            display: LcdST7789::<320, 240>::new(
                di,
                rst,
                Delay::new(),
                Self::DISPLAY_ORIENTATION,
                Self::DISPLAY_COLOUR_INVERSION,
                Self::DISPLAY_TEARING_EFFECT,
                backlight,
            )?
            .into(),
        })
    }
}

/// Marker trait only.
impl HardwareComponent for PimoroniDisplayHATMini {}

/// Mark the [`PimoroniDisplayHATMini`] as a [`DisplayComponent`].
#[async_trait]
impl DisplayComponent for PimoroniDisplayHATMini {
    type COLOUR = crate::display_mipidsi::pixelcolor::Rgb565;
    type DI = DisplaySPIInterfaceNoCS<Spi, OutputPin>;
    type MODEL = crate::display_mipidsi::screen_models::ST7789;
    type RST = OutputPin;

    const W: u16 = 320;
    const H: u16 = 240;

    /// Clear the display.
    async fn fill_display<'e>(&self, colour: Self::COLOUR) -> RPiResult<'e, ()> {
        self.display.lock().await.fill(colour)
    }
}

#[async_trait]
impl BacklightComponent for PimoroniDisplayHATMini {
    /// Turn the backlight off over an interval of time.
    async fn backlight_fade_in<'e>(
        &self,
        step: u32,
        duration: std::time::Duration,
    ) -> RPiResult<'e, ()> {
        self.display
            .lock()
            .await
            .backlight
            .transition_to(1., step, duration)
            .await
    }

    /// Turn the backlight off over an interval of time.
    async fn backlight_fade_out<'e>(
        &self,
        step: u32,
        duration: std::time::Duration,
    ) -> RPiResult<'e, ()> {
        self.display
            .lock()
            .await
            .backlight
            .transition_to(0., step, duration)
            .await
    }

    /// Turn the backlight on.
    async fn backlight_on<'e>(&self) -> RPiResult<'e, f64> {
        self.display.lock().await.backlight.set_value(1.)
    }

    /// Turn the backlight off.
    async fn backlight_off<'e>(&self) -> RPiResult<'e, f64> {
        self.display.lock().await.backlight.set_value(0.)
    }
}
