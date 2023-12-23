//! An async interface for the MIPI DSI display.
//!

use crate::foreign_types::*;
use async_trait::async_trait;
use std::sync::Arc;

use super::DisplayComponent;

#[async_trait]
pub trait UserInterface<DC>: Send + Sync
where
    DC: DisplayComponent + Sync,
{
    /// Execute the interface on the target [`LcdDisplay`].
    async fn execute<'e>(
        &self,
        display_component: &DC,
    ) -> RPiResult<'e, Option<Arc<dyn UserInterface<DC>>>>;
}
