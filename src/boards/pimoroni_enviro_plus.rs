//! Pimoroni Enviro+ on a Raspberry Pi.
//!

use rpi_display_mipidsi::{ColorInversion, Orientation};
use rpi_gpio::DisplayBacklight;
use rppal::spi::{Bus, Mode as SpiMode, SlaveSelect};
use std::marker::PhantomData;
// use rpi_display_mipidsi::LcdST7735;
use async_mutex::Mutex;

pub struct PimoroniEnviroPlus {
    // Prevents instantiation of this struct.
    _phantom: PhantomData<()>,
    // INCOMPLETE
    pub display_backlight: Mutex<DisplayBacklight>,
    // pub display: Mutex<LcdST7735>,
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
}
