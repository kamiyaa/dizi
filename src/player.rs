use std::collections::HashMap;
use std::string::ToString;
use std::time;

use serde::{Deserialize, Serialize};

use strfmt::strfmt;

use crate::error::{DiziError, DiziErrorKind, DiziResult};
use crate::playlist::{FilePlaylist, PlaylistType};
use crate::song::DiziAudioFile;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerStatus {
    Playing,
    Paused,
    Stopped,
}

impl ToString for PlayerStatus {
    fn to_string(&self) -> String {
        match *self {
            Self::Playing => "playing".to_string(),
            Self::Paused => "paused".to_string(),
            Self::Stopped => "stopped".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub song: Option<DiziAudioFile>,
    pub elapsed: time::Duration,

    pub status: PlayerStatus,
    pub playlist_status: PlaylistType,

    pub volume: usize,

    pub next: bool,
    pub repeat: bool,
    pub shuffle: bool,

    pub playlist: FilePlaylist,

    pub audio_host: String,
}

impl PlayerState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn query(&self, query: &str) -> DiziResult<String> {
        let vars = self.query_all();

        match strfmt(&query, &vars) {
            Ok(s) => Ok(s),
            Err(e) => Err(DiziError::new(
                DiziErrorKind::InvalidParameters,
                format!(
                    "Failed to process query '{}', Reason: '{}'",
                    query,
                    e.to_string()
                ),
            )),
        }
    }

    pub fn query_all(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        Self::load_player_query_vars(&mut vars, self);
        if let Some(song) = self.song.as_ref() {
            Self::load_song_query_vars(&mut vars, song);
        }
        vars
    }

    fn load_player_query_vars(vars: &mut HashMap<String, String>, player_state: &PlayerState) {
        vars.insert("player.status".to_string(), player_state.status.to_string());
        vars.insert(
            "player.volume".to_string(),
            format!("{}", player_state.volume),
        );
        vars.insert("player.next".to_string(), format!("{}", player_state.next));
        vars.insert(
            "player.repeat".to_string(),
            format!("{}", player_state.repeat),
        );
        vars.insert(
            "player.shuffle".to_string(),
            format!("{}", player_state.shuffle),
        );
        vars.insert(
            "playlist.status".to_string(),
            player_state.playlist_status.to_string(),
        );

        if let Some(index) = player_state.playlist.get_playing_index() {
            vars.insert("playlist.index".to_string(), format!("{}", index));
        }
        vars.insert(
            "playlist.length".to_string(),
            format!("{}", player_state.playlist.len()),
        );
        vars.insert("audio.host".to_string(), player_state.audio_host.clone());
    }

    fn load_song_query_vars(vars: &mut HashMap<String, String>, song: &DiziAudioFile) {
        vars.insert("song.file_name".to_string(), song.file_name().to_string());
        vars.insert(
            "song.file_path".to_string(),
            song.file_path().to_string_lossy().to_string(),
        );
        for (tag, value) in song.music_metadata().standard_tags.iter() {
            vars.insert(
                format!("song.tag.{}", tag.to_lowercase()),
                value.to_string(),
            );
        }
        if let Some(total_duration) = song.audio_metadata().total_duration.as_ref() {
            vars.insert(
                "song.total_duration".to_string(),
                total_duration.as_secs().to_string(),
            );
        }
    }
}

impl std::default::Default for PlayerState {
    fn default() -> Self {
        Self {
            song: None,
            status: PlayerStatus::Stopped,
            playlist_status: PlaylistType::PlaylistFile,
            elapsed: time::Duration::from_secs(0),
            volume: 50,
            next: true,
            repeat: false,
            shuffle: false,
            playlist: FilePlaylist::new(),
            audio_host: "UNKNOWN".to_string(),
        }
    }
}
