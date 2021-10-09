use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

use crate::macros::dizi_json;
use crate::traits::DiziJsonCommand;

pub const API_SERVER_QUIT: &str                    = "/server/quit";

dizi_json!(ServerQuit, API_SERVER_QUIT);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ServerQuit {
    pub command: String,
}
impl ServerQuit {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}
