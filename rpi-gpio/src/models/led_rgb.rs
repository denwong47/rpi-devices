//! Async structure for a physical RGB LED light connected via GPIO.
//!

use std::time::Duration;

use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use rppal::gpio::{Gpio, OutputPin};

use crate::{
    func::termination,
    traits::{CanRgbTransition, FromTupleRGB},
};
use rpi_errors::{RPiError, RPiResult};

/// A physical RGB LED light connected via GPIO.
pub struct RgbLed {
    red: OutputPin,
    green: OutputPin,
    blue: OutputPin,
    frequency: f64,

    last_value: (u8, u8, u8),
    enabled: bool,
}

impl RgbLed {
    /// Create a new RGB LED light on the given pins.
    pub fn try_new(gpio: &Gpio, red: u8, green: u8, blue: u8, frequency: f64) -> RPiResult<Self> {
        let red = gpio.get(red)?.into_output();
        let green = gpio.get(green)?.into_output();
        let blue = gpio.get(blue)?.into_output();

        let mut led = Self {
            red,
            green,
            blue,
            frequency,
            last_value: (0, 0, 0),
            enabled: false,
        };
        led.disable()?;

        Ok(led)
    }

    /// Create a new RGB LED light on the given pins; if it fails, panic.
    ///
    /// # Panics
    ///
    /// Panics if any of the pins cannot be initialized.
    pub fn new(gpio: &Gpio, red: u8, green: u8, blue: u8, frequency: f64) -> Self {
        Self::try_new(gpio, red, green, blue, frequency).expect("Failed to initialize RGB LED.")
    }

    /// Record the last value set for the LED.
    pub fn set_last_value(&mut self, red: u8, green: u8, blue: u8) -> (u8, u8, u8) {
        let mut last_value = (red, green, blue);
        std::mem::swap(&mut last_value, &mut self.last_value);

        last_value
    }

    /// Get the last value set for the LED, and set it to the LED.
    /// Used to restore the LED to its last value after it has been disabled.
    pub fn load_last_value<'e>(&mut self) -> RPiResult<'e, (u8, u8, u8)> {
        let red = self.last_value.0;
        let green = self.last_value.1;
        let blue = self.last_value.2;

        self.set_raw_values(red, green, blue)
            .map(|_| self.set_last_value(red, green, blue))
    }

    /// Get the last value set for the LED.
    pub fn values(&self) -> (u8, u8, u8) {
        self.last_value
    }

    /// Get the last value set for the LED as an [`RgbColor`].
    pub fn rgb<RGB>(&self) -> RGB
    where
        RGB: RgbColor + FromTupleRGB,
    {
        RGB::from_tuple_rgb(self.values())
    }

    /// Internal function to set the raw values for the LED without recording
    /// the last value.
    fn set_raw_values<'e>(&mut self, red: u8, green: u8, blue: u8) -> RPiResult<'e, ()> {
        macro_rules! expand_colours {
            (
                $($color:ident),*
            ) => {
                $(
                    self.$color.set_pwm_frequency(self.frequency, 1.- $color as f64 / 255.0)?;
                )*
            };
        }

        expand_colours!(red, green, blue);
        Ok(())
    }

    /// Set the values for the LED, and return the previous values.
    pub fn set_values<'e>(&mut self, red: u8, green: u8, blue: u8) -> RPiResult<'e, (u8, u8, u8)> {
        self.set_raw_values(red, green, blue)?;

        self.enabled = (red != 0) || (green != 0) || (blue != 0);

        Ok(self.set_last_value(red, green, blue))
    }

    /// Set the values for the LED using the given [`RgbColor`], and return the
    /// previous values.
    pub fn set_rgb<'e, RGB>(&mut self, rgb: &RGB) -> RPiResult<'e, RGB>
    where
        RGB: RgbColor + FromTupleRGB,
    {
        let prev_colours = self.set_values(rgb.r(), rgb.g(), rgb.b())?;

        Ok(RGB::from_rgb(
            prev_colours.0,
            prev_colours.1,
            prev_colours.2,
        ))
    }

    /// Disable the LED.
    pub fn disable<'e>(&mut self) -> RPiResult<'e, ()> {
        self.set_raw_values(0, 0, 0)?;

        self.enabled = false;

        Ok(())
    }

    /// Enable the LED.
    pub fn enable<'e>(&mut self) -> RPiResult<'e, (u8, u8, u8)> {
        let prev_colours = self.load_last_value()?;
        self.enabled = true;

        Ok(prev_colours)
    }

    /// Transition the LED to the given [`RgbColor`], using the given number of
    /// steps and duration.
    pub async fn transition_to_rgb<'e, RGB>(
        &mut self,
        dest: &RGB,
        steps: u32,
        duration: Duration,
    ) -> RPiResult<'e, ()>
    where
        RGB: RgbColor + FromTupleRGB,
    {
        if steps == 0 {
            return Err(RPiError::InvalidInput(
                "steps".into(),
                "must be greater than 0".into(),
            ));
        }
        let step_duration = duration / steps;
        let start_time = tokio::time::Instant::now();

        let source: RGB = self.rgb();

        tokio::select! {
            _ = termination::ctrl_c() => Err(RPiError::Cancelled),
            returned = async {
                for (count, rgb) in source.transition_to(dest, steps).enumerate() {
                    self.set_rgb(&rgb)?;
                    tokio::time::sleep_until(start_time + step_duration * (count as u32 + 1)).await;
                }

                Ok(())
            } => returned
        }
    }

    /// Transition the LED to the given RGB values, using the given number of
    /// steps and duration.
    pub async fn transition_to<'e>(
        &mut self,
        red: u8,
        green: u8,
        blue: u8,
        steps: u32,
        duration: Duration,
    ) -> RPiResult<'e, ()> {
        self.transition_to_rgb(&Rgb888::new(red, green, blue), steps, duration)
            .await
    }
}
