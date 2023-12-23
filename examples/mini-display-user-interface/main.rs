mod common;
pub use common::*;
mod models;
use std::sync::{Arc, OnceLock};

mod config;

static BUTTON_A: OnceLock<models::interfaces::CornerButton<'static>> = OnceLock::new();
static BUTTON_B: OnceLock<models::interfaces::CornerButton<'static>> = OnceLock::new();
static BUTTON_X: OnceLock<models::interfaces::CornerButton<'static>> = OnceLock::new();
static BUTTON_Y: OnceLock<models::interfaces::CornerButton<'static>> = OnceLock::new();

#[tokio::main]
async fn main() -> RPiResult<'static, ()> {
    let board: PimoroniDisplayHATMini = PimoroniDisplayHATMini::init()?;

    BUTTON_A
        .set(
            models::interfaces::CornerButton::from_bmp_paths(
                config::PATH_TO_ICONS.to_owned() + "/button-left-on-black.bmp",
                config::PATH_TO_ICONS.to_owned() + "/button-left-on-white.bmp",
                Arc::new(models::interfaces::DummyInterface {}),
            )
            .await?,
        )
        .expect("Failed to create button A");

    BUTTON_B
        .set(
            models::interfaces::CornerButton::from_bmp_paths(
                config::PATH_TO_ICONS.to_owned() + "/button-tick-on-black.bmp",
                config::PATH_TO_ICONS.to_owned() + "/button-tick-on-white.bmp",
                Arc::new(models::interfaces::DummyInterface {}),
            )
            .await?,
        )
        .expect("Failed to create button B");

    BUTTON_X
        .set(
            models::interfaces::CornerButton::from_bmp_paths(
                config::PATH_TO_ICONS.to_owned() + "/button-right-on-black.bmp",
                config::PATH_TO_ICONS.to_owned() + "/button-right-on-white.bmp",
                Arc::new(models::interfaces::DummyInterface {}),
            )
            .await?,
        )
        .expect("Failed to create button X");

    BUTTON_Y
        .set(
            models::interfaces::CornerButton::from_bmp_paths(
                config::PATH_TO_ICONS.to_owned() + "/button-cross-on-black.bmp",
                config::PATH_TO_ICONS.to_owned() + "/button-cross-on-white.bmp",
                Arc::new(models::interfaces::DummyInterface {}),
            )
            .await?,
        )
        .expect("Failed to create button Y");

    board
        .execute_interface_layers(Arc::new(models::interfaces::Menu::new(
            BUTTON_X.get().unwrap(),
            BUTTON_Y.get().unwrap(),
            BUTTON_A.get().unwrap(),
            BUTTON_B.get().unwrap(),
        )))
        .await
}
