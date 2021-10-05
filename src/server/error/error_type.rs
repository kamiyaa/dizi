use std::io;

use serde_derive::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct AppError {
    pub kind: String,
    pub message: String,
}

impl std::convert::From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError {
            kind: "".to_string(),
            message: err.to_string(),
        }
    }
}
