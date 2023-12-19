//! Extension of the [`HardwareComponent`] traits from [`rpi_gpio`].
//!

use super::UserInterface;
use crate::foreign_types::*;
use async_trait::async_trait;
use rpi_gpio::traits::HardwareComponent;
use std::sync::Arc;
use std::{ops::Deref, time::Duration};

/// A [`HardwareComponent`] with an [`LcdDisplay`].
#[async_trait]
pub trait DisplayComponent: HardwareComponent {
    type COLOUR: PixelColor;
    type DI: WriteOnlyDataCommand;
    type MODEL: DisplayModel<ColorFormat = Self::COLOUR>;
    type RST: OutputPinType;

    const W: u16;
    const H: u16;

    /// Clear the display.
    async fn fill_display<'e>(&self, colour: Self::COLOUR) -> RPiResult<'e, ()>;

    /// Execute the interface on the target [`DisplayComponent`].
    async fn execute_interface<'e, UI>(
        &self,
        interface: &UI,
    ) -> RPiResult<'e, Option<Arc<dyn UserInterface<Self>>>>
    where
        Self: Sized + Sync,
        UI: UserInterface<Self> + ?Sized,
    {
        interface.execute(self).await
    }

    /// Execute an interface, and if that interface returns another [`UserInterface`],
    /// execute that interface as well. If that interface returns a [`None`] instead,
    /// return to the interface of the upper level.
    async fn execute_interface_layers<'e>(
        &self,
        interface: Arc<dyn UserInterface<Self>>,
    ) -> RPiResult<'e, ()>
    where
        Self: Sized + Sync,
    {
        let mut interfaces = vec![interface];

        while let Some(current_interface) = interfaces.last() {
            let next_interface = self.execute_interface(current_interface.deref()).await?;
            if let Some(next_interface) = next_interface {
                interfaces.push(next_interface);
            } else {
                interfaces.pop();
            }
        }

        Ok(())
    }
}

#[async_trait]
pub trait BacklightComponent {
    /// Turn the backlight on over an interval of time.
    async fn backlight_fade_in<'e>(&self, step: u32, duration: Duration) -> RPiResult<'e, ()>;

    /// Turn the backlight off over an interval of time.
    async fn backlight_fade_out<'e>(&self, step: u32, duration: Duration) -> RPiResult<'e, ()>;

    /// Turn the backlight on.
    async fn backlight_on<'e>(&self) -> RPiResult<'e, f64>;

    /// Turn the backlight off.
    async fn backlight_off<'e>(&self) -> RPiResult<'e, f64>;
}

// #[async_trait]
// pub trait BacklightDisplayComponent: BacklightComponent + DisplayComponent
// {
//     /// Fade in to an interface, and fade out after the interface is complete.
//     async fn fade_into_interface<'e, UI, R>(
//         &mut self,
//         interface: &mut UI,
//         step: u32,
//         duration: Duration,
//     ) -> RPiResult<'e, UI::Return>
//     where
//         Self: Sized,
//         UI: UserInterface<Self, Return=R>,
//         R: Send,
//     {
//         let (result, _) = tokio::join!(
//             self.execute_interface(interface),
//             self.backlight_fade_in(step, duration),
//         );

//         self.backlight_fade_out(step, duration).await?;

//         result
//     }
// }

// impl<T> BacklightDisplayComponent for T
// where
//     T: BacklightComponent + DisplayComponent,
// {}
