use std::time;
use serde_derive::{Deserialize, Serialize};

use crate::playlist::{DirlistPlaylist, Playlist};
use crate::song::Song;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlayerStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PlaylistStatus {
    DirectoryListing,
    PlaylistFile,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub song: Option<Song>,
    pub elapsed: time::Duration,

    pub status: PlayerStatus,
    pub playlist_status: PlaylistStatus,

    pub volume: usize,

    pub next: bool,
    pub repeat: bool,
    pub shuffle: bool,

    pub playlist: Playlist,
}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            song: None,
            status: PlayerStatus::Stopped,
            playlist_status: PlaylistStatus::PlaylistFile,
            elapsed: time::Duration::from_secs(0),
            volume: 50,
            next: true,
            repeat: false,
            shuffle: false,
            playlist: Playlist::new(),
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

    pub fn playlist_ref(&self) -> &Playlist {
        &self.playlist
    }
    pub fn playlist_mut(&mut self) -> &mut Playlist {
        &mut self.playlist
    }
}
