#![allow(unused_imports)]

use rpi_devices::{boards::PimoroniDisplayHATMini, display_mipidsi::LcdDisplay, errors::*};
use tokio::*;

mod models;

#[tokio::main]
async fn main() -> RPiResult<'static, ()> {
    // let mut board = PimoroniDisplayHATMini::init()?;

    // let mut display = board.display.lock().await;

    Ok(())
}
