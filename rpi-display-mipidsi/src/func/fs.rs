//! Basic functions for IO operations.

use rpi_errors::{IntoRPiResult, RPiResult};
use std::path::Path;
use tokio::fs;

#[cfg(feature = "debug")]
use crate::foreign_types::logger;

/// Read bytes from file.
pub async fn read_bytes_from_file<'e>(
    path: impl AsRef<Path> + std::fmt::Debug,
) -> RPiResult<'e, Vec<u8>> {
    #[cfg(feature = "debug")]
    logger::debug(&format!("Reading bytes from {:?}...", path));

    fs::read(path).await.into_rpi_result()
}
