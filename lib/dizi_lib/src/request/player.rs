use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

use crate::macros::{dizi_json, dizi_json_stub};
use crate::traits::DiziJsonCommand;

use super::constants::*;

dizi_json_stub!(PlayerGet, API_PLAYER_GET);
dizi_json_stub!(PlayerGetVolume, API_PLAYER_GET_VOLUME);

dizi_json_stub!(PlayerPause, API_PLAYER_PAUSE);
dizi_json_stub!(PlayerResume, API_PLAYER_RESUME);

dizi_json_stub!(PlayerPlayNext, API_PLAYER_PLAY_NEXT);
dizi_json_stub!(PlayerPlayPrevious, API_PLAYER_PLAY_PREVIOUS);

dizi_json_stub!(PlayerTogglePlay, API_PLAYER_TOGGLE_PLAY);
dizi_json_stub!(PlayerToggleShuffle, API_PLAYER_TOGGLE_SHUFFLE);
dizi_json_stub!(PlayerToggleRepeat, API_PLAYER_TOGGLE_REPEAT);
dizi_json_stub!(PlayerToggleNext, API_PLAYER_TOGGLE_NEXT);

dizi_json!(PlayerFilePlay, API_PLAYER_FILE_PLAY);
dizi_json!(PlayerVolumeUp, API_PLAYER_VOLUME_UP);
dizi_json!(PlayerVolumeDown, API_PLAYER_VOLUME_DOWN);
dizi_json!(PlayerFastForward, API_PLAYER_FAST_FORWARD);
dizi_json!(PlayerRewind, API_PLAYER_REWIND);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerFilePlay {
    pub command: String,
    pub path: PathBuf,
}
impl PlayerFilePlay {
    pub fn new(path: PathBuf) -> Self {
        Self {
            command: Self::path().to_string(),
            path,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerVolumeUp {
    pub command: String,
    pub amount: usize,
}
impl PlayerVolumeUp {
    pub fn new(amount: usize) -> Self {
        Self {
            command: Self::path().to_string(),
            amount,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerVolumeDown {
    pub command: String,
    pub amount: usize,
}
impl PlayerVolumeDown {
    pub fn new(amount: usize) -> Self {
        Self {
            command: Self::path().to_string(),
            amount,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerFastForward {
    pub command: String,
    pub amount: usize,
}
impl PlayerFastForward {
    pub fn new(amount: usize) -> Self {
        Self {
            command: Self::path().to_string(),
            amount,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerRewind {
    pub command: String,
    pub amount: usize,
}
impl PlayerRewind {
    pub fn new(amount: usize) -> Self {
        Self {
            command: Self::path().to_string(),
            amount,
        }
    }
}
