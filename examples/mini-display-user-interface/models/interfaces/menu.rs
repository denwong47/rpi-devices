use super::{common, Corner, CornerButton};
use crate::{common::*, config};
use std::sync::Arc;

pub struct Menu {
    pub action_x: &'static CornerButton<'static>,
    pub action_y: &'static CornerButton<'static>,
    pub action_a: &'static CornerButton<'static>,
    pub action_b: &'static CornerButton<'static>,
}

impl Menu {
    /// Create a new [`Menu`] instance.
    pub fn new(
        action_x: &'static CornerButton<'static>,
        action_y: &'static CornerButton<'static>,
        action_a: &'static CornerButton<'static>,
        action_b: &'static CornerButton<'static>,
    ) -> Self {
        Self {
            action_x,
            action_y,
            action_a,
            action_b,
        }
    }

    /// Redraw the [`Menu`] on the display.
    pub async fn redraw<'e>(&self, hat: &PimoroniDisplayHATMini) -> RPiResult<'e, ()> {
        self.action_a
            .draw(hat, Corner::TopLeft, hat.button_a.is_pressed())
            .await?;
        self.action_b
            .draw(hat, Corner::BottomLeft, hat.button_b.is_pressed())
            .await?;
        self.action_x
            .draw(hat, Corner::TopRight, hat.button_x.is_pressed())
            .await?;
        self.action_y
            .draw(hat, Corner::BottomRight, hat.button_y.is_pressed())
            .await?;

        common::draw_mid_title(hat, "Main Menu").await?;
        common::draw_menu_lines(hat).await?;

        Ok(())
    }
}

#[async_trait]
impl UserInterface<PimoroniDisplayHATMini> for Menu {
    /// Execute the [`Menu`] user interface.
    async fn execute<'e>(
        &self,
        hat: &PimoroniDisplayHATMini,
    ) -> RPiResult<'e, Option<Arc<dyn UserInterface<PimoroniDisplayHATMini>>>> {
        hat.fill_display(Rgb565::BLACK).await?;
        self.redraw(hat).await?;
        hat.backlight_fade_in(config::FADE_IN_STEPS, config::FADE_IN_DURATION)
            .await?;

        let next = tokio::select! {
            next_a = self.action_a.handler(hat, Corner::TopLeft) => next_a,
            next_b = self.action_b.handler(hat, Corner::BottomLeft) => next_b,
            next_x = self.action_x.handler(hat, Corner::TopRight) => next_x,
            _ = self.action_y.handler(hat, Corner::BottomRight) => Ok(None),
        }?;

        hat.backlight_fade_out(config::FADE_IN_STEPS, config::FADE_IN_DURATION)
            .await?;

        Ok(next)
    }
}
