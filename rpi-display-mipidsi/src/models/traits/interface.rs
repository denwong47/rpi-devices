//! An async interface for the MIPI DSI display.
//!

use crate::{foreign_types::*, LcdDisplay};
use async_trait::async_trait;

use super::DisplayComponent;

#[async_trait]
pub trait UserInterface<DC, const W: u16, const H: u16>
where
    DC: DisplayComponent<W, H>,
{
    type Return;

    /// Execute the interface on the target [`LcdDisplay`].
    async fn execute<'e>(
        &mut self,
        display: &mut LcdDisplay<DC::DI, DC::MODEL, DC::RST, W, H>,
    ) -> RPiResult<'e, Self::Return>;
}
