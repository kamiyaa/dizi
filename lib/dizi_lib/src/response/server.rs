use std::time;

use serde_derive::{Deserialize, Serialize};

use crate::player::PlayerState;
use crate::song::Song;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ServerBroadcastEvent {
    // server is shutting down
    ServerQuit,
    ServerError { msg: String },
    ServerQuery { query: String },

    // player status updates
    PlayerState { state: PlayerState },

    PlayerFilePlay { song: Song },

    PlayerPause,
    PlayerResume,
    PlayerStop,

    PlayerRepeat { on: bool },
    PlayerShuffle { on: bool },
    PlayerNext { on: bool },

    PlayerVolumeUpdate { volume: f32 },
    PlayerProgressUpdate { elapsed: time::Duration },

    // playlist
    PlaylistOpen { state: PlayerState },
    PlaylistPlay { index: usize },
    PlaylistAppend { songs: Vec<Song> },
    PlaylistRemove { index: usize },
    PlaylistSwapMove { index1: usize, index2: usize },
    PlaylistClear,
}
