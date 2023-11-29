//! A device connected via single pin on the GPIO, and controllable via PWM.
//!
use rppal::gpio::{Gpio, OutputPin};

use crate::func::termination;
use rpi_errors::{RPiError, RPiResult};

pub struct PwmDevice {
    pin: OutputPin,
    frequency: f64,

    last_value: f64,
    enabled: bool,
}

impl PwmDevice {
    /// Create a new PWM device on the given pin.
    pub fn try_new(gpio: &Gpio, pin: u8, frequency: f64) -> RPiResult<Self> {
        let pin = gpio.get(pin)?.into_output();

        let mut device = Self {
            pin,
            frequency,
            last_value: 0.,
            enabled: false,
        };
        device.disable()?;

        Ok(device)
    }

    /// Create a new PWM device on the given pin; if it fails, panic.
    pub fn new(gpio: &Gpio, pin: u8, frequency: f64) -> Self {
        Self::try_new(gpio, pin, frequency).expect("Failed to initialize PWM device.")
    }

    /// Record the last value set for the device.
    pub fn set_last_value(&mut self, value: f64) -> f64 {
        let mut last_value = value;
        std::mem::swap(&mut last_value, &mut self.last_value);

        last_value
    }

    /// Get the last value set for the device, and set it as the current value.
    /// Used to restore the device to its last value after it has been disabled.
    pub fn load_last_value<'e>(&mut self) -> RPiResult<'e, f64> {
        let value = self.last_value;

        self.set_raw_value(value)
            .map(|_| self.set_last_value(value))
    }

    /// Get the last value set for the device.
    pub fn value(&self) -> f64 {
        self.last_value
    }

    /// Internal function to set the raw values for the LED without recording the
    /// last value.
    fn set_raw_value<'e>(&mut self, value: f64) -> RPiResult<'e, ()> {
        if !(0. ..=1.).contains(&value) {
            return Err(RPiError::InvalidInput(
                "Pwm value".into(),
                value.to_string().into(),
            ));
        }

        self.pin.set_pwm_frequency(self.frequency, value)?;

        Ok(())
    }

    /// Set the value of the device, and returns the previous value.
    pub fn set_value<'e>(&mut self, value: f64) -> RPiResult<'e, f64> {
        self.set_raw_value(value)?;
        self.enabled = value > 0.;
        Ok(self.set_last_value(value))
    }

    /// Disable the Device.
    pub fn disable<'e>(&mut self) -> RPiResult<'e, ()> {
        self.set_raw_value(0.)?;
        self.enabled = false;
        Ok(())
    }

    /// Enable the Device.
    pub fn enable<'e>(&mut self) -> RPiResult<'e, f64> {
        let prev_value = self.load_last_value()?;
        Ok(prev_value)
    }

    /// Transition the device to the given value, using the given number of steps and
    /// duration.
    pub async fn transition_to<'e>(
        &mut self,
        value: f64,
        steps: u32,
        duration: std::time::Duration,
    ) -> RPiResult<'e, ()> {
        if steps == 0 {
            return Err(RPiError::InvalidInput(
                "steps".into(),
                "must be greater than 0".into(),
            ));
        }
        if !(0. ..=1.).contains(&value) {
            return Err(RPiError::InvalidInput(
                "value".into(),
                "must be between 0 and 1".into(),
            ));
        }
        let step_duration = duration / steps;
        let start_time = tokio::time::Instant::now();

        let source = self.value();
        let step_increment = (value - source) / steps as f64;

        tokio::select! {
            _ = termination::ctrl_c() => Err(RPiError::Cancelled),
            returned = async {
                for step in 0..steps {
                    let step_time = start_time + step_duration * (step+1);
                    let step_value = source + step_increment * (step+1) as f64;
                    self.set_value(step_value)?;
                    tokio::time::sleep_until(step_time).await;
                }

                self.set_value(value).and(Ok(()))
            } => returned
        }
    }
}

/// A single frequency LED light that is dimmable.
///
/// Type alias for a [`PwmDevice`].
pub type LedPwm = PwmDevice;

/// A dimmable backlight of a display.
///
/// Type alias for a [`PwmDevice`].
pub type DisplayBacklight = PwmDevice;
