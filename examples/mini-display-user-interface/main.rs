mod common;
pub use common::*;
mod models;

mod config;

#[tokio::main]
async fn main() -> RPiResult<'static, ()> {
    let mut board = PimoroniDisplayHATMini::init()?;

    let menu = models::interfaces::Menu {
        action_x: (),
        action_y: (),
        action_a: (),
        action_b: (),
    };

    board.execute_interface_layers(Box::new(menu)).await
}
