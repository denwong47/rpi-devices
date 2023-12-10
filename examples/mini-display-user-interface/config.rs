use std::time::Duration;

pub(crate) const FADE_IN_STEPS: u32 = 32;
pub(crate) static FADE_IN_DURATION: Duration = Duration::from_millis(500);

pub(crate) const BUTTON_ICON_WIDTH: u16 = 32;
pub(crate) const BUTTON_ICON_MARGIN: u16 = 4;

pub(crate) static PATH_TO_ICONS: &str = "assets/icons";
