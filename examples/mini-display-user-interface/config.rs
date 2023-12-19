use std::time::Duration;

pub(crate) const FADE_IN_STEPS: u32 = 16;
pub(crate) static FADE_IN_DURATION: Duration = Duration::from_millis(100);

pub(crate) const BUTTON_ICON_WIDTH: u16 = 32;
pub(crate) const BUTTON_ICON_MARGIN: u16 = 4;

pub(crate) const LINE_STROKE_WIDTH: u32 = 2;

pub(crate) static PATH_TO_ICONS: &str = "examples/mini-display-user-interface/assets/icons";
