//! One of the 4 buttons on the Display HAT Mini.
//!
//!

use crate::{common::*, config};
use std::{path::Path, sync::Arc};

/// The 4 corners of the HAT.
#[derive(Clone, Copy, Debug)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Corner {
    /// Get the x coordinate of the corner.
    pub fn x(&self) -> i32 {
        (match self {
            Corner::TopLeft | Corner::BottomLeft => 0,
            _ => {
                PimoroniDisplayHATMini::W
                    - config::BUTTON_ICON_WIDTH
                    - 2 * config::BUTTON_ICON_MARGIN
            }
        }) as i32
    }

    /// Get the y coordinate of the corner.
    pub fn y(&self) -> i32 {
        (match self {
            Corner::TopLeft | Corner::TopRight => 0,
            _ => PimoroniDisplayHATMini::H / 2,
        }) as i32
    }

    // /// Get the [`Point`] of the top left corner of the [`Corner`].
    // pub fn point(&self) -> Point {
    //     Point::new(self.x(), self.y())
    // }

    /// Get the background position and size of the [`Corner`].
    pub fn bg_rect(&self) -> (Point, Size) {
        (
            Point::new(
                self.x()
                    + match &self {
                        Corner::TopLeft | Corner::BottomLeft => 0,
                        _ => config::LINE_STROKE_WIDTH,
                    } as i32,
                self.y()
                    + match &self {
                        Corner::TopLeft | Corner::TopRight => 0,
                        _ => config::LINE_STROKE_WIDTH,
                    } as i32,
            ),
            Size::new(
                (config::BUTTON_ICON_WIDTH + config::BUTTON_ICON_MARGIN * 2) as u32,
                120 + (config::BUTTON_ICON_MARGIN * 2) as u32,
            ),
        )
    }

    /// Get the centre [`Point`] of the [`Corner`] Button, offset to the lop left
    /// corner of the image.
    pub fn centre_point_offset(&self) -> Point {
        Point::new(
            self.x() + config::BUTTON_ICON_MARGIN as i32,
            self.y() + 60 - config::BUTTON_ICON_WIDTH as i32 / 2,
        )
    }
}

/// A button that occupies one of the 4 corners of the HAT.
///
/// This is not an abstraction of the physical button itself, which is done by the
/// [`rpi_devices::gpio::Button`] struct. This is mostly an instance to record the
/// state of the button and its associated display elements.
pub struct CornerButton<'i> {
    pub(crate) icons: [OwnedBmp<'i, <PimoroniDisplayHATMini as DisplayComponent>::COLOUR>; 2],
    pub(crate) interface: Arc<dyn UserInterface<PimoroniDisplayHATMini>>,
}

impl<'i> CornerButton<'i> {
    /// Create a new [`CornerButton`] instance.
    pub async fn from_bmp_paths<'e>(
        pressed_path: impl AsRef<Path> + std::fmt::Debug,
        released_path: impl AsRef<Path> + std::fmt::Debug,
        interface: Arc<dyn UserInterface<PimoroniDisplayHATMini>>,
    ) -> RPiResult<'e, Self> {
        let owned_bmps = [
            OwnedBmp::from_path(pressed_path).await?,
            OwnedBmp::from_path(released_path).await?,
        ];

        Ok(Self {
            icons: owned_bmps,
            interface,
        })
    }

    /// Draw the button on the display.
    pub async fn draw<'e>(
        &'i self,
        hat: &PimoroniDisplayHATMini,
        corner: Corner,
        pressed: bool,
    ) -> RPiResult<'e, ()> {
        let bg_colour = if pressed {
            Rgb565::WHITE
        } else {
            Rgb565::BLACK
        };

        let image =
            self.icons[if !pressed { 0 } else { 1 }].image_at(corner.centre_point_offset())?;

        {
            let mut display = hat.display.lock().await;

            let (point, size) = corner.bg_rect();
            display.draw_rect(point, size, bg_colour)?;
            display.draw_image(&image)?;
        }

        Ok(())
    }

    /// The handler for this button.
    pub async fn handler<'e>(
        &'i self,
        hat: &PimoroniDisplayHATMini,
        corner: Corner,
    ) -> RPiResult<'e, Option<Arc<dyn UserInterface<PimoroniDisplayHATMini>>>> {
        let button = match &corner {
            Corner::TopLeft => &hat.button_a,
            Corner::TopRight => &hat.button_x,
            Corner::BottomLeft => &hat.button_b,
            Corner::BottomRight => &hat.button_y,
        };

        button.pressed(None).await?;
        self.draw(hat, corner, true).await?;
        button.released(None).await?;
        self.draw(hat, corner, false).await?;

        Ok(Some(Arc::clone(&self.interface)))
    }
}

impl std::fmt::Debug for CornerButton<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CornerButton").finish()
    }
}
