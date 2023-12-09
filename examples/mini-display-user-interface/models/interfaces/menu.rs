use crate::{
    common::*,
    config::{self, BUTTON_ICON_MARGIN},
};

pub struct Menu<A, B, X, Y>
where
    A: Send,
    B: Send,
    X: Send,
    Y: Send,
{
    pub action_x: X,
    pub action_y: Y,
    pub action_a: A,
    pub action_b: B,
}

#[async_trait]
impl<A, B, X, Y> UserInterface<PimoroniDisplayHATMini> for Menu<A, B, X, Y>
where
    A: Send,
    B: Send,
    X: Send,
    Y: Send,
{
    async fn execute<'e>(
        &mut self,
        hat: &mut PimoroniDisplayHATMini,
    ) -> RPiResult<'e, Option<Box<dyn UserInterface<PimoroniDisplayHATMini>>>> {
        hat.fill_display(Rgb565::BLACK).await?;
        {
            let mut display = hat.display.lock().await;
            const BUTTON_WIDTH: i32 = (config::BUTTON_ICON_WIDTH + BUTTON_ICON_MARGIN * 2) as i32;
            const DIVIDER_HEIGHT: i32 = (PimoroniDisplayHATMini::H / 2) as i32;
            display.draw_title::<20>("Press the A button", Rgb565::WHITE, None, None)?;
            display.draw_vertical_line(BUTTON_WIDTH, Rgb565::WHITE, 2)?;
            display.draw_vertical_line(
                PimoroniDisplayHATMini::W as i32 - BUTTON_WIDTH,
                Rgb565::WHITE,
                2,
            )?;
            display.draw_line(
                Point::new(0, DIVIDER_HEIGHT),
                Point::new(BUTTON_WIDTH, DIVIDER_HEIGHT),
                Rgb565::WHITE,
                2,
            )?;
            display.draw_line(
                Point::new(PimoroniDisplayHATMini::W as i32, DIVIDER_HEIGHT),
                Point::new(
                    PimoroniDisplayHATMini::W as i32 - BUTTON_WIDTH,
                    DIVIDER_HEIGHT,
                ),
                Rgb565::WHITE,
                2,
            )?;
        }
        hat.backlight_fade_in(config::FADE_IN_STEPS, config::FADE_IN_DURATION)
            .await?;

        hat.button_a.pressed_and_released(None).await?;

        hat.backlight_fade_out(config::FADE_IN_STEPS, config::FADE_IN_DURATION)
            .await?;

        Ok(None)
    }
}
