use super::super::RPiError;
use mipidsi::error::InitError;

impl<PE> From<InitError<PE>> for RPiError<'_> {
    /// Convert a [`mipidsi::error::InitError`] into a [`RPiError`].
    fn from(value: InitError<PE>) -> Self {
        match value {
            InitError::DisplayError => RPiError::DisplayOutputError,
            InitError::Pin(_) => RPiError::DisplayInitError,
        }
    }
}
