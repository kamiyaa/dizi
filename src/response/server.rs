use std::collections::HashMap;
use std::time;

use serde::{Deserialize, Serialize};

use crate::player::PlayerState;
use crate::song::DiziAudioFile;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ServerBroadcastEvent {
    // server is shutting down
    ServerQuit,
    ServerError {
        msg: String,
    },
    ServerQuery {
        query: String,
    },
    ServerQueryAll {
        query_items: HashMap<String, String>,
    },

    // player status updates
    PlayerState {
        state: PlayerState,
    },

    PlayerFilePlay {
        file: DiziAudioFile,
    },

    PlayerPause,
    PlayerResume,
    PlayerStop,

    PlayerRepeat {
        on: bool,
    },
    PlayerShuffle {
        on: bool,
    },
    PlayerNext {
        on: bool,
    },

    PlayerVolumeUpdate {
        volume: usize,
    },
    PlayerProgressUpdate {
        elapsed: time::Duration,
    },

    // playlist
    PlaylistOpen {
        state: PlayerState,
    },
    PlaylistPlay {
        index: usize,
    },
    PlaylistAppend {
        audio_files: Vec<DiziAudioFile>,
    },
    PlaylistRemove {
        index: usize,
    },
    PlaylistSwapMove {
        index1: usize,
        index2: usize,
    },
    PlaylistClear,
}
