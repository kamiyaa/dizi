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

    pub volume: f32,

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

    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    pub fn set_volume(&mut self, volume: f32) {
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
        let vars = self.query_all();

        match strfmt(&query, &vars) {
            Ok(s) => Ok(s),
            Err(e) => Err(DiziError::new(
                DiziErrorKind::InvalidParameters,
                format!("Failed to process query '{}', Reason: '{}'", query, e.to_string()),
            )),
        }
    }

    pub fn query_all(&self) -> HashMap<String, String> {
        let mut vars = HashMap::new();
        Self::load_player_query_vars(&mut vars, self);
        if let Some(song) = self.get_song() {
            Self::load_song_query_vars(&mut vars, song);
        }
        vars
    }

    fn load_player_query_vars(vars: &mut HashMap<String, String>, player_state: &PlayerState) {
        vars.insert(
            "player.status".to_string(),
            player_state.get_player_status().to_string(),
        );
        vars.insert(
            "player.volume".to_string(),
            format!("{}", player_state.get_volume()),
        );
        vars.insert(
            "player.next".to_string(),
            format!("{}", player_state.next_enabled()),
        );
        vars.insert(
            "player.repeat".to_string(),
            format!("{}", player_state.repeat_enabled()),
        );
        vars.insert(
            "player.shuffle".to_string(),
            format!("{}", player_state.shuffle_enabled()),
        );
        vars.insert(
            "playlist.status".to_string(),
            player_state.get_playlist_status().to_string(),
        );

        if let Some(index) = player_state.playlist_ref().get_playing_index() {
            vars.insert("playlist.index".to_string(), format!("{}", index));
        }
        vars.insert(
            "playlist.length".to_string(),
            format!("{}", player_state.playlist_ref().len()),
        );
        vars.insert("audio.host".to_string(), player_state.audio_host.clone());
    }

    fn load_song_query_vars(vars: &mut HashMap<String, String>, song: &Song) {
        vars.insert("song.file_name".to_string(), song.file_name().to_string());
        vars.insert(
            "song.file_path".to_string(),
            song.file_path().to_string_lossy().to_string(),
        );
        for (tag, value) in song.music_metadata().standard_tags.iter() {
            vars.insert(format!("song.tag.{}", tag.to_lowercase()), value.to_string());
        }
        if let Some(total_duration) = song.audio_metadata().total_duration.as_ref() {
            vars.insert("song.total_duration".to_string(), total_duration.as_secs().to_string());
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
            volume: 50.0,
            next: true,
            repeat: false,
            shuffle: false,
            playlist: FilePlaylist::new(),
            audio_host: "Unknown".to_string(),
        }
    }
}
