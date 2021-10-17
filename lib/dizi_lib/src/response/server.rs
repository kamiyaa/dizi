use std::time;

use serde_derive::{Deserialize, Serialize};

use crate::song::Song;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ServerBroadcastEvent {
    // server is shutting down
    ServerQuit,

    // player status updates
    PlayerFilePlay { song: Song },

    PlayerPause,
    PlayerResume,

    PlayerRepeat { on: bool },
    PlayerShuffle { on: bool },
    PlayerNext { on: bool },

    PlayerVolumeUpdate { volume: usize },
    PlayerProgressUpdate { elapsed: time::Duration },

    PlaylistPlay { index: usize },
}
