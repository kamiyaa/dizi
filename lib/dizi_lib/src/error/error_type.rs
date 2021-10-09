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

impl std::convert::From<io::Error> for DiziError {
    fn from(err: io::Error) -> Self {
        Self {
            _kind: DiziErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

impl std::convert::From<globset::Error> for DiziError {
    fn from(err: globset::Error) -> Self {
        Self {
            _kind: DiziErrorKind::from(err.kind()),
            _cause: err.to_string(),
        }
    }
}

impl std::convert::From<std::env::VarError> for DiziError {
    fn from(err: std::env::VarError) -> Self {
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: "Environment variable not found".to_string(),
        }
    }
}

impl std::convert::From<rodio::PlayError> for DiziError {
    fn from(err: rodio::PlayError) -> Self {
        let err_str = err.to_string();
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: err_str,
        }
    }
}

impl std::convert::From<rodio::StreamError> for DiziError {
    fn from(err: rodio::StreamError) -> Self {
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: "Error with audio system".to_string(),
        }
    }
}

impl std::convert::From<rodio::decoder::DecoderError> for DiziError {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Self {
            _kind: DiziErrorKind::from(err),
            _cause: "Unsupported audio format".to_string(),
        }
    }
}

impl<T> std::convert::From<std::sync::mpsc::SendError<T>> for DiziError {
    fn from(err: std::sync::mpsc::SendError<T>) -> Self {
        Self {
            _kind: DiziErrorKind::SendError,
            _cause: "Failed to send message".to_string(),
        }
    }
}

impl<T> std::convert::From<crossbeam::channel::SendError<T>> for DiziError {
    fn from(err: crossbeam::channel::SendError<T>) -> Self {
        Self {
            _kind: DiziErrorKind::SendError,
            _cause: "Failed to send message".to_string(),
        }
    }
}
