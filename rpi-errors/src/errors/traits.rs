//! Traits for error handling.
//!
use crate::errors::RPiError;

// Convert a [`Result`] containing some other error types into a [`Result<_, RPiError>`].
pub trait IntoRPiResult<'e, T> {
    fn into_rpi_result(self) -> Result<T, RPiError<'e>>;
}
