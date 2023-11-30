use super::super::RPiError;
use display_interface::DisplayError;
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

impl From<DisplayError> for RPiError<'_> {
    /// Convert a [`display_interface::DisplayError`] into a [`RPiError`].
    fn from(value: DisplayError) -> Self {
        RPiError::DisplayInterfaceError(
            match value {
                DisplayError::InvalidFormatError => "Invalid data format selected for interface selected.".into(),
                DisplayError::BusWriteError => "Failed to write to Bus.".into(),
                DisplayError::DCError => "Unable to assert or de-assert data/command switching signal.".into(),
                DisplayError::CSError => "Unable to assert chip select signal.".into(),
                DisplayError::DataFormatNotImplemented => "The requested DataFormat is not implemented by this display interface implementation.".into(),
                DisplayError::RSError => "Unable to assert reset signal.".into(),
                DisplayError::OutOfBoundsError => "The requested pixel is outside the display area.".into(),
                err => format!("Unknown error occurred: {err:?}").into(),
            }
        )
    }
}
