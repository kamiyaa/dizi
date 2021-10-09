use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

use crate::api_command::ApiCommand;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerPlay {
    pub command: &'static str,
    pub path: PathBuf,
}

impl PlayerPlay {
    pub fn new(path: PathBuf) -> Self {
        Self {
            command: ApiCommand::PlayerPlay.to_str(),
            path,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerPause {
    pub command: &'static str,
}

impl PlayerPause {
    pub fn new() -> Self {
        Self {
            command: ApiCommand::PlayerPause.to_str(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerTogglePlay {
    pub command: &'static str,
}

impl PlayerTogglePlay {
    pub fn new() -> Self {
        Self {
            command: ApiCommand::PlayerTogglePlay.to_str(),
        }
    }
}
