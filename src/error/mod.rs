use std::io;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppErrorCode {
    Io,
    EnvVar,
    Server,
    Symphonia,

    Parse,
    SerdeJson,
    Clipboard,
    Glob,
    InvalidParameters,

    SendError,
    ReceiveError,

    Cpal,

    NoDevice,
    UnrecognizedFormat,
    NotAudioFile,

    UnrecognizedArgument,
    UnrecognizedCommand,
}

#[derive(Clone, Debug)]
pub struct AppError {
    pub code: AppErrorCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: AppErrorCode, message: String) -> Self {
        Self { code, message }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        Self::new(AppErrorCode::Io, err.to_string())
    }
}

impl From<globset::Error> for AppError {
    fn from(err: globset::Error) -> Self {
        Self::new(AppErrorCode::Glob, err.to_string())
    }
}

impl From<std::env::VarError> for AppError {
    fn from(err: std::env::VarError) -> Self {
        Self::new(AppErrorCode::EnvVar, err.to_string())
    }
}

impl From<std::sync::mpsc::RecvError> for AppError {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        Self::new(AppErrorCode::ReceiveError, err.to_string())
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for AppError {
    fn from(err: std::sync::mpsc::SendError<T>) -> Self {
        Self::new(AppErrorCode::SendError, err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        Self::new(AppErrorCode::SerdeJson, err.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        Self::new(AppErrorCode::Parse, err.to_string())
    }
}

impl From<cpal::Error> for AppError {
    fn from(err: cpal::Error) -> Self {
        Self::new(AppErrorCode::Cpal, err.to_string())
    }
}

impl From<symphonia::core::errors::Error> for AppError {
    fn from(err: symphonia::core::errors::Error) -> Self {
        Self::new(AppErrorCode::Symphonia, err.to_string())
    }
}

pub type AppResult<T = ()> = Result<T, AppError>;
