//! Pimoroni Enviro+ on a Raspberry Pi.
//!

use async_mutex::Mutex;
use async_trait::async_trait;
use rpi_display_mipidsi::{
    traits::{BacklightComponent, DisplayComponent},
    LcdST7735,
};
use rpi_display_mipidsi::{ColorInversion, DisplaySPIInterfaceNoCS, Orientation, TearingEffect};
use rpi_errors::{IntoRPiResult, RPiResult};
use rpi_gpio::{func, traits::HardwareComponent, DisplayBacklight, OutputPin};
use rppal::{
    hal::Delay,
    spi::{Bus, Mode as SpiMode, SlaveSelect, Spi},
};
use std::marker::PhantomData;

pub struct PimoroniEnviroPlus {
    // Prevents instantiation of this struct.
    _phantom: PhantomData<()>,
    // INCOMPLETE
    pub display: Mutex<LcdST7735<160, 80, true>>,
}

impl PimoroniEnviroPlus {
    pub const SPI_BUS: Bus = Bus::Spi0;
    pub const SPI_SLAVE: SlaveSelect = SlaveSelect::Ss1;
    pub const SPI_CLOCK_SPEED: u32 = 10_000_000;
    pub const SPI_MODE: SpiMode = SpiMode::Mode0;

    pub const SPI_MOSI: u8 = 10;
    pub const SPI_DC: u8 = 9;
    pub const SPI_SLCK: u8 = 11;
    pub const SPI_CS: u8 = 7;
    pub const DISPLAY_BACKLIGHT: u8 = 12;

    pub const DISPLAY_ORIENTATION: Orientation = Orientation::LandscapeInverted(true);
    pub const DISPLAY_COLOUR_INVERSION: ColorInversion = ColorInversion::Inverted;
    pub const DISPLAY_TEARING_EFFECT: TearingEffect = TearingEffect::Off;
    pub const DISPLAY_RESET: Option<u8> = None;

    pub const I2C_SDA: u8 = 2;
    pub const I2C_SCL: u8 = 3;

    pub const PMS5003_UART_0: u8 = 14;
    pub const PMS5003_UART_1: u8 = 15;

    pub const PMS5003_RESET: u8 = 27;
    pub const PMS5003_ENABLE: u8 = 22;

    pub const ADS1015_ALERT: u8 = 23;
    pub const MICS6814_HEATER: u8 = 24;

    pub const MIC_I2S_FS: u8 = 19;
    pub const MIC_I2S_DATA: u8 = 20;

    /// Initialize the Pimoroni Enviro+.
    ///
    /// # Note
    ///
    /// Creating two instances of this struct is undefined behaviour, and will likely
    /// fail.
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
            display: LcdST7735::<160, 80, true>::new(
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
impl HardwareComponent for PimoroniEnviroPlus {}

/// Mark the [`PimoroniDisplayHATMini`] as a [`DisplayComponent`].
#[async_trait]
impl DisplayComponent for PimoroniEnviroPlus {
    type COLOUR = crate::display_mipidsi::pixelcolor::Bgr565;
    type DI = DisplaySPIInterfaceNoCS<Spi, OutputPin>;
    type MODEL = crate::display_mipidsi::screen_models::ST7735;
    type RST = OutputPin;

    const W: u16 = 160;
    const H: u16 = 80;

    /// Clear the display.
    async fn fill_display<'e>(&mut self, colour: Self::COLOUR) -> RPiResult<'e, ()> {
        self.display.lock().await.fill(colour)
    }
}

#[async_trait]
impl BacklightComponent for PimoroniEnviroPlus {
    /// Turn the backlight off over an interval of time.
    async fn backlight_fade_in<'e>(
        &mut self,
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
        &mut self,
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
    async fn backlight_on<'e>(&mut self) -> RPiResult<'e, f64> {
        self.display.lock().await.backlight.set_value(1.)
    }

    /// Turn the backlight off.
    async fn backlight_off<'e>(&mut self) -> RPiResult<'e, f64> {
        self.display.lock().await.backlight.set_value(0.)
    }
}
