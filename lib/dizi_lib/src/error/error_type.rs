use std::convert::From;
use std::io;

use super::DiziErrorKind;

#[derive(Debug)]
pub struct DiziError {
    _kind: DiziErrorKind,
    _cause: String,
}

impl DiziError {
    pub fn new(_kind: DiziErrorKind, _cause: String) -> Self {
        Self { _kind, _cause }
    }

    pub fn kind(&self) -> &DiziErrorKind {
        &self._kind
    }
}

impl std::fmt::Display for DiziError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self._cause)
    }
}

impl From<io::Error> for DiziError {
    fn from(err: io::Error) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err.kind()),
            _cause,
        }
    }
}

impl From<globset::Error> for DiziError {
    fn from(err: globset::Error) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err.kind()),
            _cause,
        }
    }
}

impl From<std::env::VarError> for DiziError {
    fn from(err: std::env::VarError) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

impl From<std::sync::mpsc::RecvError> for DiziError {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::ReceiveError,
            _cause,
        }
    }
}

#[cfg(feature = "rodio-backend")]
impl From<rodio::decoder::DecoderError> for DiziError {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

#[cfg(feature = "rodio-backend")]
impl From<rodio::PlayError> for DiziError {
    fn from(err: rodio::PlayError) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

#[cfg(feature = "rodio-backend")]
impl From<rodio::StreamError> for DiziError {
    fn from(err: rodio::StreamError) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for DiziError {
    fn from(err: std::sync::mpsc::SendError<T>) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::SendError,
            _cause,
        }
    }
}

impl From<serde_json::Error> for DiziError {
    fn from(err: serde_json::Error) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

impl From<toml::de::Error> for DiziError {
    fn from(err: toml::de::Error) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

#[cfg(feature = "symphonia-backend")]
impl From<symphonia::core::errors::Error> for DiziError {
    fn from(err: symphonia::core::errors::Error) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

impl From<cpal::BuildStreamError> for DiziError {
    fn from(err: cpal::BuildStreamError) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

impl From<cpal::PlayStreamError> for DiziError {
    fn from(err: cpal::PlayStreamError) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}

impl From<cpal::PauseStreamError> for DiziError {
    fn from(err: cpal::PauseStreamError) -> Self {
        let _cause = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause,
        }
    }
}
