//! Convert a [`Result`] containing some other error types into a [`Result<_, RPiError>`].
//!

use crate::errors::{IntoRPiResult, RPiError};

impl<'e, T, E> IntoRPiResult<'e, T> for Result<T, E>
where
    E: Into<RPiError<'e>>,
{
    /// Convert a [`Result`] containing some other error types into a [`Result<_, RPiError>`].
    fn into_rpi_result(self) -> Result<T, RPiError<'e>> {
        self.map_err(|e| e.into())
    }
}
