use std::path::PathBuf;

use serde_derive::{Deserialize, Serialize};

use crate::macros::dizi_json;
use crate::traits::DiziJsonCommand;

use super::constants::*;

dizi_json!(PlayerGet, API_PLAYER_GET);
dizi_json!(PlayerPlay, API_PLAYER_PLAY);
dizi_json!(PlayerPause, API_PLAYER_PAUSE);
dizi_json!(PlayerResume, API_PLAYER_RESUME);
dizi_json!(PlayerTogglePlay, API_PLAYER_TOGGLE_PLAY);
dizi_json!(PlayerToggleShuffle, API_PLAYER_TOGGLE_SHUFFLE);
dizi_json!(PlayerToggleRepeat, API_PLAYER_TOGGLE_REPEAT);
dizi_json!(PlayerToggleNext, API_PLAYER_TOGGLE_NEXT);
dizi_json!(PlayerGetVolume, API_PLAYER_GET_VOLUME);
dizi_json!(PlayerVolumeUp, API_PLAYER_VOLUME_UP);
dizi_json!(PlayerVolumeDown, API_PLAYER_VOLUME_DOWN);
dizi_json!(PlayerFastForward, API_PLAYER_FAST_FORWARD);
dizi_json!(PlayerRewind, API_PLAYER_REWIND);
dizi_json!(PlayerNext, API_PLAYER_NEXT);
dizi_json!(PlayerPrevious, API_PLAYER_PREVIOUS);


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerGet {
    pub command: String,
}
impl PlayerGet {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerTogglePlay {
    pub command: String,
}
impl PlayerTogglePlay {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerToggleShuffle {
    pub command: String,
}
impl PlayerToggleShuffle {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerToggleRepeat {
    pub command: String,
}
impl PlayerToggleRepeat {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerToggleNext {
    pub command: String,
}
impl PlayerToggleNext {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerGetVolume {
    pub command: String,
}
impl PlayerGetVolume {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerNext {
    pub command: String,
}
impl PlayerNext {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerPrevious {
    pub command: String,
}
impl PlayerPrevious {
    pub fn new() -> Self {
        Self {
            command: Self::path().to_string(),
        }
    }
}
