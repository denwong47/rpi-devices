/*
# SPI ST7789 on a Raspberry Pi Zero W Example

This example demonstrates how to use the [Display HAT Mini by Pomoroni](https://shop.pimoroni.com/products/display-hat-mini?variant=39496084717651)
on a Raspberry Pi Zero W.

The example shows a scrolling text and a pulsing RGB LED.

Buttons:

- A: change LED color
- B: exit
- X: move text up
- Y: move text down

Read the README.md for more information.
*/

use std::{thread::sleep, time::Duration};
use display_interface_spi::SPIInterfaceNoCS;
use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::*,
    text::Text,
};
use std::fs;
use mipidsi::Builder;
use rppal::gpio::{Gpio, OutputPin};
use rppal::hal::Delay;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};
use std::process::ExitCode;

// Pins

const SPI_DC: u8 = 9;
const BACKLIGHT: u8 = 13;

const BUTTON_A: u8 = 5;
const BUTTON_B: u8 = 6;
const BUTTON_X: u8 = 16;
const BUTTON_Y: u8 = 24;

const LED_R: u8 = 17;
const LED_G: u8 = 27;
const LED_B: u8 = 22;

// Display
const W: i32 = 320;
const H: i32 = 240;

fn main() -> ExitCode {
    // GPIO
    let gpio = Gpio::new().unwrap();
    let dc = gpio.get(SPI_DC).unwrap().into_output();
    let mut backlight = gpio.get(BACKLIGHT).unwrap().into_output();

    // LEDs
    let mut led_r = gpio.get(LED_R).unwrap().into_output();
    let mut led_g = gpio.get(LED_G).unwrap().into_output();
    let mut led_b = gpio.get(LED_B).unwrap().into_output();

    // Buttons
    let button_a = gpio.get(BUTTON_A).unwrap().into_input_pullup();
    let button_b = gpio.get(BUTTON_B).unwrap().into_input_pullup();
    let button_x = gpio.get(BUTTON_X).unwrap().into_input_pullup();
    let button_y = gpio.get(BUTTON_Y).unwrap().into_input_pullup();

    // SPI Display
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss1, 60_000_000_u32, Mode::Mode0).unwrap();
    let di = SPIInterfaceNoCS::new(spi, dc);
    let mut delay = Delay::new();
    let mut display = Builder::st7789(di)
        // width and height are switched on purpose because of the oritation
        .with_display_size(H as u16, W as u16)        // this orientation applies for the Display HAT Mini by Pimoro
        .with_orientation(mipidsi::Orientation::LandscapeInverted(true))
        .with_invert_colors(mipidsi::ColorInversion::Inverted)
        .init(&mut delay, None::<OutputPin>)
        .unwrap();

    // Text
    let char_w = 10;
    let char_h = 20;
    let text_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
    let text = "Happy life with Wifey";
    let mut text_x = W;
    let mut text_y = H / 2;

    // Image
    let image_srcs = [
        "/home/denwong47/Documents/bus.bin",
        "/home/denwong47/Documents/landscape.bin",
        "/home/denwong47/Documents/tank.bin",
    ];
    let image_bytes = image_srcs.iter().map(
        |path| {
            fs::read(path).unwrap()
        }
    ).collect::<Vec<_>>();
    let image_raws = image_bytes.iter().map(
        |bytes| {
            ImageRaw::<'_, Rgb565>::new(&bytes, W as u32)
        }
    ).collect::<Vec<_>>();
    let images = image_raws.iter().map(
        |raw| {
            Image::new(raw, Point::zero())
        }
    ).collect::<Vec<_>>();

    // Clear the display initially
    display.clear(Rgb565::BLACK).unwrap();

    // Turn on backlight
    backlight.set_pwm_frequency(50., 0.75).unwrap();

    // Set LEDs to PWM mode
    led_r.set_pwm_frequency(50., 1.).unwrap();
    led_g.set_pwm_frequency(50., 1.).unwrap();
    led_b.set_pwm_frequency(50., 1.).unwrap();

    let start = std::time::Instant::now();
    let mut last = std::time::Instant::now();
    let mut led_flags = 0b000;
    let mut counter = 0;
    loop {
        let elapsed = last.elapsed().as_secs_f64();
        if elapsed < 0.250 {
            sleep(Duration::from_millis(120));
            continue;
        }
        last = std::time::Instant::now();
        counter += 1;

        // X: move text up
        if button_x.is_low() {
            text_y -= char_h;
        }
        // Y: move text down
        if button_y.is_low() {
            text_y += char_h;
        }
        // A: change led color
        if button_a.is_low() {
            led_flags = (led_flags + 1) % 8;
        }
        // B: exit
        if button_b.is_low() {
            break;
        }

        // Fill the display with alternating colors every 8 frames
        images[(counter / 8) % images.len()].draw(&mut display).unwrap();

        // Draw text
        let right = Text::new(text, Point::new(text_x, text_y), text_style)
            .draw(&mut display)
            .unwrap();
        text_x = if right.x <= 0 { W } else { text_x - char_w };

        // Led
        let y = ((start.elapsed().as_secs_f64().sin() + 1.) * 50.).round() / 100.;
        led_r
            .set_pwm_frequency(50., if led_flags & 0b100 != 0 { y } else { 1. })
            .unwrap();
        led_g
            .set_pwm_frequency(50., if led_flags & 0b010 != 0 { y } else { 1. })
            .unwrap();
        led_b
            .set_pwm_frequency(50., if led_flags & 0b001 != 0 { y } else { 1. })
            .unwrap();
    }

    // Turn off backlight and clear the display
    backlight.set_low();
    display.clear(Rgb565::BLACK).unwrap();

    ExitCode::SUCCESS
}
