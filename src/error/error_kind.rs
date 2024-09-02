use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum DiziErrorKind {
    Server,
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

    SymphoniaError(symphonia::core::errors::Error),

    CpalBuildStreamError(cpal::BuildStreamError),
    CpalPlayStreamError(cpal::PlayStreamError),
    CpalPauseStreamError(cpal::PauseStreamError),

    NoDevice,
    UnrecognizedFormat,
    NotAudioFile,

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

impl From<cpal::PauseStreamError> for DiziErrorKind {
    fn from(e: cpal::PauseStreamError) -> Self {
        Self::CpalPauseStreamError(e)
    }
}
