use std::collections::HashMap;
use std::string::ToString;
use std::time;

use serde_derive::{Deserialize, Serialize};

use strfmt::strfmt;

use crate::error::{DiziError, DiziErrorKind, DiziResult};
use crate::playlist::{FilePlaylist, PlaylistType};
use crate::song::Song;

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
    pub song: Option<Song>,
    pub elapsed: time::Duration,

    pub status: PlayerStatus,
    pub playlist_status: PlaylistType,

    pub volume: usize,

    pub next: bool,
    pub repeat: bool,
    pub shuffle: bool,

    pub playlist: FilePlaylist,
}

impl PlayerState {
    pub fn new() -> Self {
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
        }
    }

    pub fn get_song(&self) -> Option<&Song> {
        self.song.as_ref()
    }
    pub fn set_song(&mut self, song: Option<Song>) {
        self.song = song;
    }

    pub fn get_elapsed(&self) -> time::Duration {
        self.elapsed
    }
    pub fn set_elapsed(&mut self, elapsed: time::Duration) {
        self.elapsed = elapsed;
    }

    pub fn get_player_status(&self) -> PlayerStatus {
        self.status
    }
    pub fn set_player_status(&mut self, status: PlayerStatus) {
        self.status = status;
    }
    pub fn get_playlist_status(&self) -> PlaylistType {
        self.playlist_status
    }
    pub fn set_playlist_status(&mut self, status: PlaylistType) {
        self.playlist_status = status;
    }

    pub fn get_volume(&self) -> usize {
        self.volume
    }
    pub fn set_volume(&mut self, volume: usize) {
        self.volume = volume;
    }

    pub fn repeat_enabled(&self) -> bool {
        self.repeat
    }
    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }

    pub fn shuffle_enabled(&self) -> bool {
        self.shuffle
    }
    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.shuffle = shuffle;
    }

    pub fn next_enabled(&self) -> bool {
        self.next
    }
    pub fn set_next(&mut self, next: bool) {
        self.next = next;
    }

    pub fn playlist_ref(&self) -> &FilePlaylist {
        &self.playlist
    }
    pub fn playlist_mut(&mut self) -> &mut FilePlaylist {
        &mut self.playlist
    }

    pub fn query(&self, query: &str) -> DiziResult<String> {
        let player_state = self;

        let mut vars = HashMap::new();

        vars.insert(
            "player_status".to_string(),
            player_state.get_player_status().to_string(),
        );
        vars.insert(
            "player_volume".to_string(),
            format!("{}", player_state.get_volume()),
        );
        vars.insert(
            "player_next".to_string(),
            format!("{}", player_state.next_enabled()),
        );
        vars.insert(
            "player_repeat".to_string(),
            format!("{}", player_state.repeat_enabled()),
        );
        vars.insert(
            "player_shuffle".to_string(),
            format!("{}", player_state.shuffle_enabled()),
        );

        if let Some(song) = player_state.get_song() {
            vars.insert("file_name".to_string(), song.file_name().to_string());
            vars.insert(
                "file_path".to_string(),
                song.file_path().to_string_lossy().to_string(),
            );
        }

        vars.insert(
            "playlist_status".to_string(),
            player_state.get_playlist_status().to_string(),
        );

        if let Some(index) = player_state.playlist_ref().get_playing_index() {
            vars.insert("playlist_index".to_string(), format!("{}", index));
        }
        vars.insert(
            "playlist_length".to_string(),
            format!("{}", player_state.playlist_ref().len()),
        );

        match strfmt(&query, &vars) {
            Ok(s) => Ok(s),
            Err(_e) => Err(DiziError::new(
                DiziErrorKind::InvalidParameters,
                "Failed to process query".to_string(),
            )),
        }
    }
}
