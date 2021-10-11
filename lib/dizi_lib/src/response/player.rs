use std::path::PathBuf;
use std::time;

use serde_derive::{Deserialize, Serialize};

use crate::macros::{dizi_json, dizi_json_stub};
use crate::song::Song;
use crate::traits::DiziJsonCommand;

use super::constants::*;

dizi_json_stub!(PlayerPause, RESP_PLAYER_PAUSE);
dizi_json_stub!(PlayerResume, RESP_PLAYER_RESUME);
dizi_json_stub!(PlayerShuffleOn, RESP_PLAYER_SHUFFLE_ON);
dizi_json_stub!(PlayerShuffleOff, RESP_PLAYER_SHUFFLE_OFF);
dizi_json_stub!(PlayerRepeatOn, RESP_PLAYER_REPEAT_ON);
dizi_json_stub!(PlayerRepeatOff, RESP_PLAYER_REPEAT_OFF);
dizi_json_stub!(PlayerNextOn, RESP_PLAYER_NEXT_ON);
dizi_json_stub!(PlayerNextOff, RESP_PLAYER_NEXT_OFF);

dizi_json!(PlayerPlay, RESP_PLAYER_PLAY);
dizi_json!(PlayerVolumeUpdate, RESP_PLAYER_VOLUME_UPDATE);
dizi_json!(PlayerProgressUpdate, RESP_PLAYER_PROGRESS_UPDATE);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerPlay {
    pub command: String,
    pub song: Song,
}
impl PlayerPlay {
    pub fn new(song: Song) -> Self {
        Self {
            command: Self::path().to_string(),
            song,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerVolumeUpdate {
    pub command: String,
    pub volume: usize,
}
impl PlayerVolumeUpdate {
    pub fn new(volume: usize) -> Self {
        Self {
            command: Self::path().to_string(),
            volume,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerProgressUpdate {
    pub command: String,
    pub duration: time::Duration,
}
impl PlayerProgressUpdate {
    pub fn new(duration: time::Duration) -> Self {
        Self {
            command: Self::path().to_string(),
            duration,
        }
    }
}

