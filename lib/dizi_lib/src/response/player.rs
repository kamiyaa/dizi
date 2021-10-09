use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

use crate::macros::dizi_json;
use crate::traits::DiziJsonCommand;

use super::constants::*;

dizi_json!(PlayerPlay, RESP_PLAYER_PLAY);
dizi_json!(PlayerPause, RESP_PLAYER_PAUSE);
dizi_json!(PlayerResume, RESP_PLAYER_RESUME);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerPlay {
    pub command: String,
    pub path: PathBuf,
}
impl PlayerPlay {
    pub fn new(path: PathBuf) -> Self {
        Self {
            command: Self::path().to_string(),
            path,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerPause {
    pub command: String,
}
impl PlayerPause {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerResume {
    pub command: String,
}
impl PlayerResume {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}
