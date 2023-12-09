//! An async interface for the MIPI DSI display.
//!

use crate::foreign_types::*;
use async_trait::async_trait;

use super::DisplayComponent;

#[async_trait]
pub trait UserInterface<DC>: Send
where
    DC: DisplayComponent,
{
    /// Execute the interface on the target [`LcdDisplay`].
    async fn execute<'e>(
        &mut self,
        display_component: &mut DC,
    ) -> RPiResult<'e, Option<Box<dyn UserInterface<DC>>>>;
}
