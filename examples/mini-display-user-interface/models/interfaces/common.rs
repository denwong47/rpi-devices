use crate::{common::*, config};

/// Draw the menu lines on the display.
pub(crate) async fn draw_menu_lines<'e>(hat: &PimoroniDisplayHATMini) -> RPiResult<'e, ()> {
    const BUTTON_WIDTH: i32 = (config::BUTTON_ICON_WIDTH + config::BUTTON_ICON_MARGIN * 2) as i32;
    const DIVIDER_HEIGHT: i32 = (PimoroniDisplayHATMini::H / 2) as i32;

    let mut display = hat.display.lock().await;

    display.draw_vertical_line(BUTTON_WIDTH, Rgb565::WHITE, config::LINE_STROKE_WIDTH)?;
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

    Ok(())
}

/// Clear the body of the display.
pub(crate) async fn clear_body<'e>(hat: &PimoroniDisplayHATMini) -> RPiResult<'e, ()> {
    let mut display = hat.display.lock().await;

    display.draw_rect(
        Point::new(
            config::BUTTON_ICON_WIDTH as i32
                + config::BUTTON_ICON_MARGIN as i32 * 2
                + config::LINE_STROKE_WIDTH as i32 / 2,
            0,
        ),
        Size::new(
            (PimoroniDisplayHATMini::W as i32
                - config::BUTTON_ICON_MARGIN as i32 * 4
                - config::BUTTON_ICON_WIDTH as i32 * 2
                - config::LINE_STROKE_WIDTH as i32) as u32,
            PimoroniDisplayHATMini::H as u32,
        ),
        Rgb565::BLACK,
    )?;

    Ok(())
}

/// Draw a title in the middle of the screen.
pub(crate) async fn draw_mid_title<'e>(
    hat: &PimoroniDisplayHATMini,
    title: &str,
) -> RPiResult<'e, ()> {
    let mut display = hat.display.lock().await;

    display.draw_title::<20>(
        title,
        Rgb565::WHITE,
        Some(Point::new(
            config::BUTTON_ICON_WIDTH as i32
                + config::BUTTON_ICON_MARGIN as i32 * 2
                + config::LINE_STROKE_WIDTH as i32 / 2,
            PimoroniDisplayHATMini::H as i32 / 2 - 10,
        )),
        Some(Size::new(
            (PimoroniDisplayHATMini::W as i32
                - config::BUTTON_ICON_MARGIN as i32 * 4
                - config::BUTTON_ICON_WIDTH as i32 * 2
                - config::LINE_STROKE_WIDTH as i32) as u32,
            PimoroniDisplayHATMini::H as u32,
        )),
    )?;

    Ok(())
}
