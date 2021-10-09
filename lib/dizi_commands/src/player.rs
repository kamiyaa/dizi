use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

pub const API_PLAYER_GET: &str              = "/player/get";
pub const API_PLAYER_PLAY: &str             = "/player/play";
pub const API_PLAYER_PAUSE: &str            = "/player/pause";
pub const API_PLAYER_RESUME: &str           = "/player/resume";

pub const API_PLAYER_TOGGLE_PLAY: &str      = "/player/toggle/play";
pub const API_PLAYER_TOGGLE_SHUFFLE: &str   = "/player/toggle/shuffle";
pub const API_PLAYER_TOGGLE_REPEAT: &str    = "/player/toggle/repeat";
pub const API_PLAYER_TOGGLE_NEXT: &str      = "/player/toggle/next";

pub const API_PLAYER_GET_VOLUME: &str       = "/player/volume/get";
pub const API_PLAYER_VOLUME_UP: &str        = "/player/volume/increase";
pub const API_PLAYER_VOLUME_DOWN: &str      = "/player/volume/decrease";

pub const API_PLAYER_REWIND: &str           = "/player/rewind";
pub const API_PLAYER_FAST_FORWARD: &str     = "/player/fastforward";

pub trait DiziJsonCommand<'a>: serde::Deserialize<'a> + serde::Serialize {
    fn path() -> &'static str;
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PlayerPlay {
    pub command: String,
    pub path: PathBuf,
}
impl DiziJsonCommand<'static> for PlayerPlay {
    fn path() -> &'static str {
        API_PLAYER_PLAY
    }
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
impl DiziJsonCommand<'static> for PlayerPause {
    fn path() -> &'static str {
        API_PLAYER_PAUSE
    }
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
impl DiziJsonCommand<'static> for PlayerResume {
    fn path() -> &'static str {
        API_PLAYER_RESUME
    }
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
impl DiziJsonCommand<'static> for PlayerTogglePlay {
    fn path() -> &'static str {
        API_PLAYER_TOGGLE_PLAY
    }
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
impl DiziJsonCommand<'static> for PlayerToggleShuffle {
    fn path() -> &'static str {
        API_PLAYER_TOGGLE_SHUFFLE
    }
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
impl DiziJsonCommand<'static> for PlayerToggleRepeat {
    fn path() -> &'static str {
        API_PLAYER_TOGGLE_REPEAT
    }
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
impl DiziJsonCommand<'static> for PlayerToggleNext {
    fn path() -> &'static str {
        API_PLAYER_TOGGLE_NEXT
    }
}
impl PlayerToggleNext {
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
impl DiziJsonCommand<'static> for PlayerVolumeUp {
    fn path() -> &'static str {
        API_PLAYER_VOLUME_UP
    }
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
impl DiziJsonCommand<'static> for PlayerVolumeDown {
    fn path() -> &'static str {
        API_PLAYER_VOLUME_DOWN
    }
}
impl PlayerVolumeDown {
    pub fn new(amount: usize) -> Self {
        Self {
            command: Self::path().to_string(),
            amount,
        }
    }
}
