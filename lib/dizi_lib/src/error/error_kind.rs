use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum DiziErrorKind {
    // io related
    IoError(io::ErrorKind),

    // environment variable not found
    EnvVarNotPresent,

    // parse error
    ParseError,
    SerdeJson,
    ClipboardError,

    Glob,
    InvalidParameters,

    SendError,
    ReceiveError,

    CpalBuildStreamError(cpal::BuildStreamError),
    CpalPlayStreamError(cpal::PlayStreamError),

    DecoderError(rodio::decoder::DecoderError),
    SymphoniaError(symphonia::core::errors::Error),
    NoDevice,
    UnrecognizedFormat,
    NotAudioFile,
    StreamError(rodio::StreamError),

    UnrecognizedArgument,
    UnrecognizedCommand,
}

impl From<io::ErrorKind> for DiziErrorKind {
    fn from(err: io::ErrorKind) -> Self {
        Self::IoError(err)
    }
}

impl From<&globset::ErrorKind> for DiziErrorKind {
    fn from(_: &globset::ErrorKind) -> Self {
        Self::Glob
    }
}

impl From<std::env::VarError> for DiziErrorKind {
    fn from(_: std::env::VarError) -> Self {
        Self::EnvVarNotPresent
    }
}

impl From<rodio::PlayError> for DiziErrorKind {
    fn from(err: rodio::PlayError) -> Self {
        match err {
            rodio::PlayError::DecoderError(e) => Self::DecoderError(e),
            rodio::PlayError::NoDevice => Self::NoDevice,
        }
    }
}

impl From<rodio::StreamError> for DiziErrorKind {
    fn from(err: rodio::StreamError) -> Self {
        Self::StreamError(err)
    }
}

impl From<rodio::decoder::DecoderError> for DiziErrorKind {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Self::DecoderError(err)
    }
}

impl From<serde_json::Error> for DiziErrorKind {
    fn from(_: serde_json::Error) -> Self {
        Self::SerdeJson
    }
}

impl From<toml::de::Error> for DiziErrorKind {
    fn from(_: toml::de::Error) -> Self {
        Self::ParseError
    }
}

impl From<symphonia::core::errors::Error> for DiziErrorKind {
    fn from(e: symphonia::core::errors::Error) -> Self {
        Self::SymphoniaError(e)
    }
}

impl From<cpal::BuildStreamError> for DiziErrorKind {
    fn from(e: cpal::BuildStreamError) -> Self {
        Self::CpalBuildStreamError(e)
    }
}

impl From<cpal::PlayStreamError> for DiziErrorKind {
    fn from(e: cpal::PlayStreamError) -> Self {
        Self::CpalPlayStreamError(e)
    }
}
