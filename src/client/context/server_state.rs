use std::time;

use dizi_lib::player::PlayerStatus;
use dizi_lib::playlist::Playlist;
use dizi_lib::song::Song;

#[derive(Clone, Debug)]
pub struct Player {
    song: Option<Song>,
    player_status: PlayerStatus,
    duration_played: time::Duration,
    volume: usize,
    repeat: bool,
    shuffle: bool,
    next: bool,
    playlist: Playlist,
}

impl Player {
    pub fn new() -> Self {
        Self {
            song: None,
            player_status: PlayerStatus::Stopped,
            duration_played: time::Duration::from_secs(0),
            volume: 100,
            repeat: false,
            next: false,
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

    pub fn get_duration_played(&self) -> time::Duration {
        self.duration_played
    }
    pub fn set_duration_played(&mut self, duration_played: time::Duration) {
        self.duration_played = duration_played;
    }

    pub fn get_player_status(&self) -> PlayerStatus {
        self.player_status
    }
    pub fn set_player_status(&mut self, player_status: PlayerStatus) {
        self.player_status = player_status;
    }

    pub fn get_volume(&self) -> usize {
        self.volume
    }
    pub fn set_volume(&mut self, volume: usize) {
        self.volume = volume;
    }

    pub fn get_repeat(&self) -> bool {
        self.repeat
    }
    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }

    pub fn get_shuffle(&self) -> bool {
        self.shuffle
    }
    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.shuffle = shuffle;
    }

    pub fn get_next(&self) -> bool {
        self.next
    }
    pub fn set_next(&mut self, next: bool) {
        self.next = next;
    }
}

#[derive(Clone, Debug)]
pub struct ServerState {
    player: Player,
}

impl ServerState {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
        }
    }

    pub fn player_ref(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }
}
