use super::super::RPiError;

use tinybmp::ParseError;

impl From<ParseError> for RPiError<'_> {
    /// Convert a [`tinybmp::ParseError`] into a [`RPiError`].
    fn from(value: ParseError) -> Self {
        RPiError::BMPError(
            match value {
                ParseError::UnsupportedBpp(bit_depth) => format!("Unsupported Bitmap bit depth of {bit_depth}.").into(),
                ParseError::UnexpectedEndOfFile => "File terminateed unexpectedly".into(),
                ParseError::InvalidFileSignature(bytes) => format!("Invalid file signature: Bitmap files must start with `BM` ([66, 77]), but {bytes:?} found.").into(),
                ParseError::UnsupportedCompressionMethod(method) => format!("Compression method {method} is not supported.").into(),
                ParseError::UnsupportedHeaderLength(length) => format!("Header length declaredd as {length}, which is not supported.").into(),
                ParseError::UnsupportedChannelMasks => "Found unsupported Bitmap Channel masks.".into(),
                ParseError::InvalidImageDimensions => "Found invalid image dimensions in Bitmap.".into(),
                // err => format!("Unknown error occurred: {err:?}").into(),
            }
        )
    }
}
