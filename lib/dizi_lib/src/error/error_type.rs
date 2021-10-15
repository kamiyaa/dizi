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
        Self {
            _kind: DiziErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

impl From<globset::Error> for DiziError {
    fn from(err: globset::Error) -> Self {
        Self {
            _kind: DiziErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

impl From<std::env::VarError> for DiziError {
    fn from(err: std::env::VarError) -> Self {
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: "Environment variable not found".to_string(),
        }
    }
}

impl From<rodio::PlayError> for DiziError {
    fn from(err: rodio::PlayError) -> Self {
        let err_str = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: err_str,
        }
    }
}

impl From<rodio::StreamError> for DiziError {
    fn from(err: rodio::StreamError) -> Self {
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: "Error with audio system".to_string(),
        }
    }
}

impl From<rodio::decoder::DecoderError> for DiziError {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: "Unsupported audio format".to_string(),
        }
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for DiziError {
    fn from(err: std::sync::mpsc::SendError<T>) -> Self {
        Self {
            _kind: DiziErrorKind::SendError,
            _cause: "Failed to send message".to_string(),
        }
    }
}

impl<T> From<crossbeam::channel::SendError<T>> for DiziError {
    fn from(err: crossbeam::channel::SendError<T>) -> Self {
        Self {
            _kind: DiziErrorKind::SendError,
            _cause: "Failed to send message".to_string(),
        }
    }
}

impl From<serde_json::Error> for DiziError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: "Failed to parse JSON".to_string(),
        }
    }
}

#[cfg(feature = "ffmpeg")]
impl From<ffmpeg_decoder::Error> for DiziError {
    fn from(err: ffmpeg_decoder::Error) -> Self {
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: "Unsupported audio format".to_string(),
        }
    }
}
