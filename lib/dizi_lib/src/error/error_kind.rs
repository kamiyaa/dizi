use std::io;

#[derive(Debug)]
pub enum DiziErrorKind {
    // io related
    IoError(io::ErrorKind),

    // environment variable not found
    EnvVarNotPresent,

    // parse error
    ParseError,
    ClipboardError,

    Glob,
    InvalidParameters,

    SendError,

    DecoderError,
    NoDevice,
    UnrecognizedFormat,
    StreamError(rodio::StreamError),

    UnrecognizedArgument,
    UnrecognizedCommand,
}

impl std::convert::From<io::ErrorKind> for DiziErrorKind {
    fn from(err: io::ErrorKind) -> Self {
        Self::IoError(err)
    }
}

impl std::convert::From<&globset::ErrorKind> for DiziErrorKind {
    fn from(_: &globset::ErrorKind) -> Self {
        Self::Glob
    }
}

impl std::convert::From<std::env::VarError> for DiziErrorKind {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvVarNotPresent
    }
}

impl std::convert::From<rodio::PlayError> for DiziErrorKind {
    fn from(err: rodio::PlayError) -> Self {
        match err {
            rodio::PlayError::DecoderError(_) => Self::DecoderError,
            rodio::PlayError::NoDevice => Self::NoDevice,
        }
    }
}

impl std::convert::From<rodio::StreamError> for DiziErrorKind {
    fn from(err: rodio::StreamError) -> Self {
        Self::StreamError(err)
    }
}

impl std::convert::From<rodio::decoder::DecoderError> for DiziErrorKind {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Self::UnrecognizedFormat
    }
}
