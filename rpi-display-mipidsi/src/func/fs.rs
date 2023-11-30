//! Basic functions for IO operations.
use rpi_errors::{IntoRPiResult, RPiResult};
use std::path::Path;
use tokio::fs;

/// Read bytes from file.
pub async fn read_bytes_from_file<'e>(path: impl AsRef<Path>) -> RPiResult<'e, Vec<u8>> {
    fs::read(path).await.into_rpi_result()
}
