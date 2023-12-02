//! An async interface for the MIPI DSI display.
//!

use crate::{foreign_types::*, LcdDisplay};
use async_trait::async_trait;

#[async_trait]
pub trait UserInterface<DI, MODEL, RST, const W: u16, const H: u16>
where
    DI: WriteOnlyDataCommand,
    MODEL: DisplayModel,
    RST: OutputPinType,
    MODEL::ColorFormat: From<<MODEL::ColorFormat as PixelColor>::Raw>,
{
    type Return;

    /// Execute the interface on the target [`LcdDisplay`].
    async fn execute<'e>(
        &mut self,
        display: &mut LcdDisplay<DI, MODEL, RST, W, H>,
    ) -> RPiResult<'e, Self::Return>;
}
