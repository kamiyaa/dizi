use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time;

use log::{debug, log_enabled, Level};

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};
use dizi_lib::player::{PlayerState, PlayerStatus};
use dizi_lib::playlist::PlaylistStatus;
use dizi_lib::song::Song;

use crate::audio::{
    player_stream, PlayerDirectoryPlaylist, PlayerFilePlaylist, PlayerPlaylist, PlayerRequest,
};
use crate::config;
use crate::events::ServerEventSender;
use crate::util::mimetype::{get_mimetype, is_mimetype_audio, is_mimetype_video};

#[derive(Debug)]
pub struct Player {
    current_song: Option<Song>,
    elapsed: time::Duration,

    status: PlayerStatus,

    volume: f32,

    shuffle: bool,
    repeat: bool,
    next: bool,

    playlist: PlayerPlaylist,

    event_tx: ServerEventSender,

    player_handle: thread::JoinHandle<DiziResult<()>>,
    player_req_tx: mpsc::Sender<PlayerRequest>,
    player_res_rx: mpsc::Receiver<DiziResult<()>>,
}

impl Player {
    pub fn new(config_t: &config::AppConfig, event_tx: ServerEventSender) -> Self {
        let (player_req_tx, player_req_rx) = mpsc::channel();
        let (player_res_tx, player_res_rx) = mpsc::channel();

        let volume = config_t.server_ref().player_ref().volume as f32 / 100.0;
        let config_t2 = config_t.clone();
        let event_tx2 = event_tx.clone();
        let player_handle = thread::spawn(move || {
            let res = player_stream(volume, config_t2, player_res_tx, player_req_rx, event_tx2);
            match res.as_ref() {
                Ok(_) => {}
                Err(e) => {
                    if log_enabled!(Level::Debug) {
                        debug!("PlayerStream: {:?}", e);
                    }
                }
            }
            res
        });

        let server_config = config_t.server_ref();
        let player_config = server_config.player_ref();

        let mut playlist = PlayerPlaylist::default();
        playlist.file_playlist =
            PlayerFilePlaylist::from_file(&PathBuf::from("/"), server_config.playlist_ref())
                .unwrap_or_else(|_| PlayerFilePlaylist::new(Vec::new()));

        Self {
            current_song: None,
            elapsed: time::Duration::from_secs(0),

            status: PlayerStatus::Stopped,
            volume,

            shuffle: player_config.shuffle,
            repeat: player_config.repeat,
            next: player_config.next,

            playlist,

            event_tx,

            player_handle,
            player_req_tx,
            player_res_rx,
        }
    }

    pub fn clone_player_state(&self) -> PlayerState {
        let song = self.current_song_ref().map(|s| s.clone());
        let elapsed = self.get_elapsed();
        let status = self.play_status();
        let playlist_status = self.playlist_ref().status;
        let volume: usize = (self.get_volume() * 100.0) as usize;
        let shuffle = self.shuffle_enabled();
        let next = self.next_enabled();
        let repeat = self.repeat_enabled();

        let playlist = self.playlist_ref().file_playlist.clone_file_playlist();

        PlayerState {
            song,
            elapsed,

            status,
            playlist_status,

            volume,

            next,
            repeat,
            shuffle,

            playlist,
        }
    }

    fn player_stream_req(&self) -> &mpsc::Sender<PlayerRequest> {
        &self.player_req_tx
    }
    fn player_stream_res(&self) -> &mpsc::Receiver<DiziResult<()>> {
        &self.player_res_rx
    }

    fn play(&mut self, song: &Song) -> DiziResult<()> {
        self.player_stream_req()
            .send(PlayerRequest::Play(song.clone()))?;
        let _resp = self.player_stream_res().recv()??;

        self.status = PlayerStatus::Playing;
        self.current_song = Some(song.clone());
        Ok(())
    }

    pub fn play_entire_directory(&mut self, path: &Path) -> DiziResult<()> {
        let mimetype = get_mimetype(path)?;
        if !is_mimetype_audio(&mimetype) && !is_mimetype_video(&mimetype) {
            return Err(DiziError::new(
                DiziErrorKind::NotAudioFile,
                format!("File mimetype is not of type audio: '{}'", mimetype),
            ));
        }
        let song = Song::new(path)?;

        let shuffle_enabled = self.shuffle_enabled();
        if let Some(parent) = song.file_path().parent() {
            let mut directory_playlist = PlayerDirectoryPlaylist::from_path(parent)?;
            // find the song we're playing in the playlist and set playing index
            // equal to the playing song
            let index = directory_playlist
                .songs_ref()
                .iter()
                .enumerate()
                .find(|(_, p)| p.file_path() == path)
                .map(|(i, _)| i);
            if let Some(index) = index {
                directory_playlist.set_song_index(index);
            }
            if shuffle_enabled {
                directory_playlist.on_spot_shuffle();
            }
            self.play(&song)?;
            self.playlist.directory_playlist = directory_playlist;
            self.playlist.set_status(PlaylistStatus::DirectoryListing);
        }
        Ok(())
    }

    pub fn play_from_playlist(&mut self, index: usize) -> DiziResult<()> {
        let shuffle_enabled = self.shuffle_enabled();
        let playlist = self.playlist_mut().file_playlist_mut();

        playlist.set_song_index(index);
        if shuffle_enabled {
            playlist.on_spot_shuffle();
        }
        if let Some(song_index) = playlist.get_song_index() {
            let song = playlist.songs_ref()[song_index].clone();
            self.play(&song)?;
            self.playlist.set_status(PlaylistStatus::PlaylistFile);
        }
        Ok(())
    }

    pub fn play_from_directory(&mut self, index: usize) -> DiziResult<()> {
        let playlist = self.playlist_mut().directory_playlist_mut();

        playlist.set_song_index(index);
        if let Some(song_index) = playlist.get_song_index() {
            let song = playlist.songs_ref()[song_index].clone();
            self.play(&song)?;
            self.playlist.set_status(PlaylistStatus::DirectoryListing);
        }
        Ok(())
    }

    pub fn play_again(&mut self) -> DiziResult<()> {
        match self.playlist_ref().status {
            PlaylistStatus::PlaylistFile => {
                let playlist = self.playlist_mut().file_playlist_mut();
                if let Some(song_index) = playlist.get_song_index() {
                    let song = playlist.songs_ref()[song_index].clone();
                    self.play(&song)?;
                }
            }
            PlaylistStatus::DirectoryListing => {
                let playlist = self.playlist_mut().directory_playlist_mut();
                if let Some(song_index) = playlist.get_song_index() {
                    let song = playlist.songs_ref()[song_index].clone();
                    self.play(&song)?;
                }
            }
        }
        Ok(())
    }

    pub fn play_next(&mut self) -> DiziResult<()> {
        match self.playlist_ref().status {
            PlaylistStatus::PlaylistFile => {
                let playlist = self.playlist_mut().file_playlist_mut();
                if playlist.is_empty() {
                    return Ok(());
                }
                playlist.increment_order_index();
                if let Some(song_index) = playlist.get_song_index() {
                    if song_index < playlist.len() {
                        let song = playlist.songs_ref()[song_index].clone();
                        self.play(&song)?;
                    } else if let Some(order_index) = playlist.get_order_index() {
                        playlist.playlist_order_mut().remove(order_index);
                        playlist.decrement_order_index();
                        self.play_next();
                    }
                }
            }
            PlaylistStatus::DirectoryListing => {
                let playlist = self.playlist_mut().directory_playlist_mut();
                if playlist.is_empty() {
                    return Ok(());
                }
                playlist.increment_order_index();
                if let Some(song_index) = playlist.get_song_index() {
                    if song_index < playlist.len() {
                        let song = playlist.songs_ref()[song_index].clone();
                        self.play(&song)?;
                    } else if let Some(order_index) = playlist.get_order_index() {
                        playlist.playlist_order_mut().remove(order_index);
                        playlist.decrement_order_index();
                        self.play_next();
                    }
                }
            }
        }
        Ok(())
    }

    pub fn play_previous(&mut self) -> DiziResult<()> {
        match self.playlist_ref().status {
            PlaylistStatus::PlaylistFile => {
                let playlist = self.playlist_mut().file_playlist_mut();
                playlist.decrement_order_index();
                if let Some(song_index) = playlist.get_song_index() {
                    let song = playlist.songs_ref()[song_index].clone();
                    self.play(&song)?;
                }
            }
            PlaylistStatus::DirectoryListing => {
                let playlist = self.playlist_mut().directory_playlist_mut();
                playlist.decrement_order_index();
                if let Some(song_index) = playlist.get_song_index() {
                    let song = playlist.songs_ref()[song_index].clone();
                    self.play(&song)?;
                }
            }
        }
        Ok(())
    }

    pub fn pause(&mut self) -> DiziResult<()> {
        self.player_stream_req().send(PlayerRequest::Pause)?;

        self.player_stream_res().recv()??;
        self.status = PlayerStatus::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> DiziResult<()> {
        self.player_stream_req().send(PlayerRequest::Resume)?;

        self.player_stream_res().recv()??;
        self.status = PlayerStatus::Playing;
        Ok(())
    }

    pub fn toggle_play(&mut self) -> DiziResult<PlayerStatus> {
        match self.status {
            PlayerStatus::Playing => {
                self.pause()?;
                Ok(PlayerStatus::Paused)
            }
            PlayerStatus::Paused => {
                self.resume()?;
                Ok(PlayerStatus::Playing)
            }
            s => Ok(s),
        }
    }
    pub fn play_status(&self) -> PlayerStatus {
        self.status
    }

    pub fn get_volume(&self) -> f32 {
        self.volume
    }
    pub fn set_volume(&mut self, volume: f32) -> DiziResult<()> {
        self.player_stream_req()
            .send(PlayerRequest::SetVolume(volume))?;

        self.player_stream_res().recv()??;
        self.volume = volume;
        Ok(())
    }
    pub fn next_enabled(&self) -> bool {
        self.next
    }
    pub fn repeat_enabled(&self) -> bool {
        self.repeat
    }
    pub fn shuffle_enabled(&self) -> bool {
        self.shuffle
    }

    pub fn set_next(&mut self, next: bool) {
        self.next = next;
    }
    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }
    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.shuffle = shuffle;
        if self.shuffle {
            self.playlist_mut().file_playlist_mut().on_spot_shuffle();
            self.playlist_mut()
                .directory_playlist_mut()
                .on_spot_shuffle();
        }
    }

    pub fn get_elapsed(&self) -> time::Duration {
        self.elapsed
    }
    pub fn set_elapsed(&mut self, elapsed: time::Duration) {
        self.elapsed = elapsed;
    }

    pub fn current_song_ref(&self) -> Option<&Song> {
        self.current_song.as_ref()
    }

    pub fn playlist_ref(&self) -> &PlayerPlaylist {
        &self.playlist
    }
    pub fn playlist_mut(&mut self) -> &mut PlayerPlaylist {
        &mut self.playlist
    }
}
