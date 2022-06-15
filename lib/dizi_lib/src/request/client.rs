use std::path::PathBuf;
use std::time;

use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "api")]
pub enum ClientRequest {
    // quit server
    #[serde(rename = "/server/quit")]
    ServerQuit,
    #[serde(rename = "/server/query")]
    ServerQuery { query: String },

    // client left
    #[serde(rename = "/client/leave")]
    ClientLeave { uuid: String },

    // player requests
    #[serde(rename = "/player/state")]
    PlayerState,
    #[serde(rename = "/player/play/file")]
    PlayerFilePlay { path: Option<PathBuf> },

    #[serde(rename = "/player/play/next")]
    PlayerPlayNext,
    #[serde(rename = "/player/play/previous")]
    PlayerPlayPrevious,

    #[serde(rename = "/player/pause")]
    PlayerPause,
    #[serde(rename = "/player/resume")]
    PlayerResume,
    #[serde(rename = "/player/volume/get")]
    PlayerGetVolume,

    #[serde(rename = "/player/rewind")]
    PlayerRewind { amount: time::Duration },
    #[serde(rename = "/player/fast_forward")]
    PlayerFastForward { amount: time::Duration },

    #[serde(rename = "/player/toggle/play")]
    PlayerTogglePlay,
    #[serde(rename = "/player/toggle/next")]
    PlayerToggleNext,
    #[serde(rename = "/player/toggle/repeat")]
    PlayerToggleRepeat,
    #[serde(rename = "/player/toggle/shuffle")]
    PlayerToggleShuffle,

    #[serde(rename = "/player/volume/increase")]
    PlayerVolumeUp { amount: usize },
    #[serde(rename = "/player/volume/decrease")]
    PlayerVolumeDown { amount: usize },

    // playlist requests
    #[serde(rename = "/playlist/state")]
    PlaylistState,
    #[serde(rename = "/playlist/open")]
    PlaylistOpen {
        cwd: Option<PathBuf>,
        path: Option<PathBuf>,
    },
    #[serde(rename = "/playlist/play")]
    PlaylistPlay { index: Option<usize> },

    #[serde(rename = "/playlist/append")]
    PlaylistAppend { path: Option<PathBuf> },
    #[serde(rename = "/playlist/remove")]
    PlaylistRemove { index: Option<usize> },
    #[serde(rename = "/playlist/clear")]
    PlaylistClear,
    #[serde(rename = "/playlist/move_up")]
    PlaylistMoveUp { index: Option<usize> },
    #[serde(rename = "/playlist/move_down")]
    PlaylistMoveDown { index: Option<usize> },
}

impl ClientRequest {
    pub fn api_path(&self) -> &'static str {
        match &*self {
            Self::ClientLeave { .. } => "/client/leave",
            Self::ServerQuit => "/server/quit",
            Self::ServerQuery { .. } => "/server/query",

            Self::PlayerState => "/player/state",
            Self::PlayerFilePlay { .. } => "/player/play/file",
            Self::PlayerPlayNext => "/player/play/next",
            Self::PlayerPlayPrevious => "/player/play/previous",
            Self::PlayerPause => "/player/pause",
            Self::PlayerResume => "/player/resume",
            Self::PlayerGetVolume => "/player/volume/get",
            Self::PlayerRewind { .. } => "/player/rewind",
            Self::PlayerFastForward { .. } => "/player/fast_forward",
            Self::PlayerTogglePlay => "/player/toggle/play",
            Self::PlayerToggleNext => "/player/toggle/next",
            Self::PlayerToggleRepeat => "/player/toggle/repeat",
            Self::PlayerToggleShuffle => "/player/toggle/shuffle",
            Self::PlayerVolumeUp { .. } => "/player/volume/increase",
            Self::PlayerVolumeDown { .. } => "/player/volume/decrease",

            Self::PlaylistState => "/playlist/state",
            Self::PlaylistOpen { .. } => "/playlist/open",
            Self::PlaylistPlay { .. } => "/playlist/play",

            Self::PlaylistAppend { .. } => "/playlist/append",
            Self::PlaylistRemove { .. } => "/playlist/remove",
            Self::PlaylistClear => "/playlist/clear",

            Self::PlaylistMoveUp { .. } => "/playlist/move_up",
            Self::PlaylistMoveDown { .. } => "/playlist/move_down",
        }
    }
}
