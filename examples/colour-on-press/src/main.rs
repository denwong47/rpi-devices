use rpi_devices::{boards::PimoroniDisplayHATMini, errors::RPiResult};

pub mod models;
use models::traits::ColourOnPress;

#[tokio::main]
async fn main() -> RPiResult<'static, ()> {
    let mut unit = PimoroniDisplayHATMini::init()?;

    unit.run().await
}
