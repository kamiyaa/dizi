use crate::constants::*;
use crate::error::{DiziError, DiziErrorKind};

#[derive(Copy, Clone, Debug)]
pub enum ApiCommand {
    Quit,

    PlaylistGet,
    PlaylistAdd,
    PlaylistRemove,

    PlayerGet,
    PlayerPlay,
    PlayerPause,
    PlayerTogglePlay,
    PlayerToggleShuffle,
    PlayerToggleRepeat,
    PlayerToggleNext,

    PlayerVolumeUp,
    PlayerVolumeDown,

    PlayerRewind,
    PlayerFastForward,
}

impl ApiCommand {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Quit => API_QUIT,

            Self::PlaylistGet => API_PLAYLIST_GET,
            Self::PlaylistAdd => API_PLAYLIST_ADD,
            Self::PlaylistRemove => API_PLAYLIST_REMOVE,

            Self::PlayerGet => API_PLAYER_GET,
            Self::PlayerPlay => API_PLAYER_PLAY,
            Self::PlayerPause => API_PLAYER_PAUSE,

            Self::PlayerTogglePlay => API_PLAYER_TOGGLE_PLAY,
            Self::PlayerToggleShuffle => API_PLAYER_TOGGLE_SHUFFLE,
            Self::PlayerToggleRepeat => API_PLAYER_TOGGLE_REPEAT,
            Self::PlayerToggleNext => API_PLAYER_TOGGLE_NEXT,

            Self::PlayerVolumeUp => API_PLAYER_VOLUME_UP,
            Self::PlayerVolumeDown => API_PLAYER_VOLUME_DOWN,

            Self::PlayerRewind => API_PLAYER_REWIND,
            Self::PlayerFastForward => API_PLAYER_FAST_FORWARD,
        }
    }
}

impl std::str::FromStr for ApiCommand {
    type Err = DiziError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            API_QUIT => Ok(Self::Quit),

            API_PLAYLIST_GET => Ok(Self::PlaylistGet),
            API_PLAYLIST_ADD => Ok(Self::PlaylistAdd),
            API_PLAYLIST_REMOVE => Ok(Self::PlaylistRemove),

            API_PLAYER_GET => Ok(Self::PlayerGet),
            API_PLAYER_PLAY => Ok(Self::PlayerPlay),
            API_PLAYER_PAUSE => Ok(Self::PlayerPause),

            API_PLAYER_TOGGLE_PLAY => Ok(Self::PlayerTogglePlay),
            API_PLAYER_TOGGLE_SHUFFLE => Ok(Self::PlayerToggleShuffle),
            API_PLAYER_TOGGLE_REPEAT => Ok(Self::PlayerToggleRepeat),
            API_PLAYER_TOGGLE_NEXT => Ok(Self::PlayerToggleNext),

            API_PLAYER_VOLUME_UP => Ok(Self::PlayerVolumeUp),
            API_PLAYER_VOLUME_DOWN => Ok(Self::PlayerVolumeDown),

            API_PLAYER_REWIND => Ok(Self::PlayerRewind),
            API_PLAYER_FAST_FORWARD => Ok(Self::PlayerFastForward),

            command => Err(DiziError::new(
                DiziErrorKind::UnrecognizedCommand,
                format!("Unrecognized command '{}'", command),
            ))
        }
    }
}
