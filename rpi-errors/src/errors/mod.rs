//! Error types.
//!

use std::{borrow::Cow, sync::LockResult, time::Duration};

use rppal::spi::Error as SpiError;

use thiserror::Error;

mod implementations;
mod traits;
pub use traits::IntoRPiResult;

/// Error type
#[derive(Error, Debug)]
pub enum RPiError<'e> {
    /// Invalid input; first argument is the variable name, second argument is the input.
    #[error("Invalid input for {0}: {1}")]
    InvalidInput(Cow<'e, str>, Cow<'e, str>),

    #[error("Lock Poisoned when {0}")]
    Poisoned(Cow<'e, str>),

    #[error("Tokio reported an async Error: {0}")]
    AsyncError(#[from] tokio::task::JoinError),

    #[error("Raspberry Pi System level Error: {0}")]
    System(Cow<'e, str>),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("GPIO Error: {0}")]
    GPIO(#[from] rppal::gpio::Error),

    #[error("SPI Error: {0}")]
    SPI(#[from] SpiError),

    #[error("Operation to {0} timed out after {1:?}.")]
    Timeout(Cow<'e, str>, Duration),

    #[error("Operation Cancelled.")]
    Cancelled,

    /// Catch all errors
    #[error("Unexpected error: {0}")]
    Unknown(Cow<'e, str>),

    #[cfg(feature = "display")]
    #[error("Display failed to initialise.")]
    DisplayInitError,

    #[cfg(feature = "display")]
    #[error("Failed to display content.")]
    DisplayOutputError,

    #[cfg(feature = "display")]
    #[error("Display interface reported an error: {0}")]
    DisplayInterfaceError(Cow<'e, str>),

    #[cfg(feature = "bmp")]
    #[error("Failed to parse BMP image: {0}")]
    BMPError(Cow<'e, str>),
}

/// Result type with the error being [`RPiError`].
pub type RPiResult<'e, T> = Result<T, RPiError<'e>>;

impl<'e> RPiError<'e> {
    /// Create a new [`RPiError::Poisoned`] error from a [`LockResult`].
    pub fn from_poison_result<T>(result: LockResult<T>, operation: &'e str) -> RPiResult<'e, T> {
        result.map_err(|_| RPiError::Poisoned(operation.into()))
    }
}
