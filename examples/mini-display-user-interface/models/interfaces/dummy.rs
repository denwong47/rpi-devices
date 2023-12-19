use crate::{common::*, config};
use std::sync::Arc;
use std::time::Duration;

use super::common;

pub struct DummyInterface {}

#[async_trait]
impl UserInterface<PimoroniDisplayHATMini> for DummyInterface {
    async fn execute<'e>(
        &self,
        hat: &PimoroniDisplayHATMini,
    ) -> RPiResult<'e, Option<Arc<dyn UserInterface<PimoroniDisplayHATMini>>>> {
        hat.fill_display(Rgb565::BLACK).await?;
        common::clear_body(hat).await?;
        common::draw_menu_lines(hat).await?;

        for i in 0..5 {
            let remaining = 4 - i;

            common::draw_mid_title(hat, &format!("Wait.. {}s", remaining)).await?;

            if i == 0 {
                hat.backlight_fade_in(config::FADE_IN_STEPS, config::FADE_IN_DURATION)
                    .await?;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
            common::clear_body(hat).await?;
        }

        hat.backlight_fade_out(config::FADE_IN_STEPS, config::FADE_IN_DURATION)
            .await?;

        Ok(None)
    }
}
