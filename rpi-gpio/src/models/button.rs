//! Async structure for a physical button connected via GPIO.
//!

use rppal::gpio::{Gpio, InputPin};
use std::time::Duration;

use crate::func::termination;
use rpi_errors::{RPiError, RPiResult};

/// A physical button connected via GPIO.
pub struct Button {
    pin: InputPin,
}

impl Button {
    /// Create a new button on the given pin.
    pub fn try_new(gpio: &Gpio, pin: u8) -> RPiResult<Self> {
        let pin = gpio.get(pin)?.into_input_pullup();
        Ok(Self { pin })
    }

    /// Create a new button on the given pin with the High/Low states inverted.
    pub fn try_new_inverted(gpio: &Gpio, pin: u8) -> RPiResult<Self> {
        let pin = gpio.get(pin)?.into_input_pulldown();
        Ok(Self { pin })
    }

    /// Create a new button on the given pin; if it fails, panic.
    ///
    /// # Panics
    ///
    /// Panics if the pin cannot be initialized.
    pub fn new(gpio: &Gpio, pin: u8) -> Self {
        Self::try_new(gpio, pin).unwrap()
    }

    /// Create a new button on the given pin with the High/Low states inverted;
    /// if it fails, panic.
    ///
    /// # Panics
    pub fn new_inverted(gpio: &Gpio, pin: u8) -> Self {
        Self::try_new_inverted(gpio, pin).unwrap()
    }

    /// Returns `true`` if the button is pressed.
    pub fn is_pressed(&self) -> bool {
        self.pin.is_low()
    }

    /// Blocks until the button is pressed, then eturns `true`.
    pub async fn wait_until_pressed_state<'e>(
        &self,
        state: bool,
        timeout: Option<Duration>,
    ) -> RPiResult<'e, bool> {
        tokio::select! {
            state = async {
                loop {
                    if self.is_pressed() == state {
                        return state;
                    }
                    tokio::task::yield_now().await;
                }
            } => Ok(state),
            _ = termination::timeout_opt(timeout) => {
                let state = if state { "pressed" } else { "released" };
                // You can never timeout if timeout is `None`. So this is safe.
                Err(RPiError::Timeout(format!("wait for button {state}").into(), timeout.unwrap()))
            },
            _ = termination::ctrl_c() => Err(RPiError::Cancelled),
        }
    }

    /// Blocks until the button is pressed, then eturns `true`.
    pub async fn pressed<'e>(&self, timeout: Option<Duration>) -> RPiResult<'e, bool> {
        self.wait_until_pressed_state(true, timeout).await
    }

    /// Executes `func` and returns the return value when the button is pressed.
    pub async fn pressed_then<'e, T>(
        &self,
        func: impl Fn(RPiResult<'e, bool>) -> RPiResult<'e, T>,
        timeout: Option<Duration>,
    ) -> RPiResult<'e, T> {
        func(self.pressed(timeout).await)
    }

    /// Returns `true` when the button is released.
    pub async fn released<'e>(&self, timeout: Option<Duration>) -> RPiResult<'e, bool> {
        self.wait_until_pressed_state(false, timeout).await
    }

    /// Wait until the button is pressed for the first time, and then released;
    /// then returns `true`.
    pub async fn pressed_and_released<'e>(&self, timeout: Option<Duration>) -> RPiResult<'e, bool> {
        self.pressed(timeout).await?;
        self.released(timeout).await
    }

    /// Wait until the button is pressed for the first time, perform a callback;
    /// then another when released. Finally return the final return value of the
    /// callback.
    pub async fn pressed_and_released_then<'e, T>(
        &self,
        func: impl Fn(RPiResult<'e, bool>) -> RPiResult<'e, T>,
        timeout: Option<Duration>,
    ) -> RPiResult<'e, T> {
        let start_time = tokio::time::Instant::now();

        func(self.pressed(timeout).await)?;

        // Reduce the timeout by the time it took to press the button.
        let timeout = timeout.map(|t| t - start_time.elapsed());

        self.pressed_then(func, timeout).await
    }

    /// Infinitely loop the button press callback - execute the callback when
    /// pressed and released, then repeat.
    pub async fn release_event_loop<'e, T>(
        &self,
        func: impl Fn(RPiResult<'e, bool>) -> RPiResult<'e, T>,
    ) -> RPiResult<'e, T> {
        loop {
            func(self.pressed(None).await)?;
            func(self.released(None).await)?;
        }
    }
}
