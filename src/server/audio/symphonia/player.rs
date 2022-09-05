use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time;

use log::{debug, log_enabled, Level};

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};
use dizi_lib::player::{PlayerState, PlayerStatus};
use dizi_lib::playlist::PlaylistType;
use dizi_lib::song::Song;

use crate::audio::request::PlayerRequest;
use crate::config;
use crate::context::PlaylistContext;
use crate::events::ServerEventSender;
use crate::playlist::playlist_directory::PlaylistDirectory;
use crate::playlist::playlist_file::PlaylistFile;
use crate::playlist::traits::{OrderedPlaylist, ShufflePlaylist};
use crate::util::mimetype::{get_mimetype, is_mimetype_audio, is_mimetype_video};

use super::stream::init_player_stream;

#[derive(Debug)]
pub struct Player {
    state: PlayerState,

    playlist_context: PlaylistContext,

    event_tx: ServerEventSender,

    player_handle: thread::JoinHandle<DiziResult>,
    player_req_tx: mpsc::Sender<PlayerRequest>,
    player_res_rx: mpsc::Receiver<DiziResult>,
}

impl Player {
    pub fn new(config_t: &config::AppConfig, event_tx: ServerEventSender) -> Self {
        let (player_req_tx, player_req_rx) = mpsc::channel();
        let (player_res_tx, player_res_rx) = mpsc::channel();

        let config_t2 = config_t.clone();
        let event_tx2 = event_tx.clone();
        let player_handle = thread::spawn(move || {
            let res = init_player_stream(player_res_tx, player_req_rx, event_tx2);
            Ok(())
        });

        let server_config = config_t.server_ref();
        let player_config = server_config.player_ref();

        let mut playlist_context = PlaylistContext::default();
        playlist_context.file_playlist =
            PlaylistFile::from_file(&PathBuf::from("/"), server_config.playlist_ref())
                .unwrap_or_else(|_| PlaylistFile::new(Vec::new()));

        let mut state = PlayerState::default();
        state.next = player_config.next;
        state.repeat = player_config.repeat;
        state.shuffle = player_config.shuffle;
        let volume = config_t.server_ref().player_ref().volume;
        state.volume = volume;

        Self {
            state,

            playlist_context,

            event_tx,

            player_handle,
            player_req_tx,
            player_res_rx,
        }
    }

    fn player_stream_req(&self) -> &mpsc::Sender<PlayerRequest> {
        &self.player_req_tx
    }
    fn player_stream_res(&self) -> &mpsc::Receiver<DiziResult> {
        &self.player_res_rx
    }

    pub fn player_state(&self) -> &PlayerState {
        &self.state
    }

    fn play(&mut self, song: &Song) -> DiziResult {
        self.player_stream_req()
            .send(PlayerRequest::Play(song.clone()))?;
        let _resp = self.player_stream_res().recv()??;

        self.state.status = PlayerStatus::Playing;
        self.state.song = Some(song.clone());
        Ok(())
    }

    pub fn play_again(&mut self) -> DiziResult {
        if let Some(entry) = self.playlist_ref().current_entry_details() {
            self.play(&entry.entry)?;
        }
        Ok(())
    }

    pub fn play_next(&mut self) -> DiziResult {
        if let Some(entry) = self.playlist_mut().next_song() {
            self.play(&entry.entry)?;
        }
        Ok(())
    }

    pub fn play_previous(&mut self) -> DiziResult {
        if let Some(entry) = self.playlist_mut().previous_song() {
            self.play(&entry.entry)?;
        }
        Ok(())
    }

    pub fn pause(&mut self) -> DiziResult {
        self.player_stream_req().send(PlayerRequest::Pause)?;

        self.player_stream_res().recv()??;
        self.state.status = PlayerStatus::Paused;
        Ok(())
    }

    pub fn resume(&mut self) -> DiziResult {
        self.player_stream_req().send(PlayerRequest::Resume)?;

        self.player_stream_res().recv()??;
        self.state.status = PlayerStatus::Playing;
        Ok(())
    }

    pub fn stop(&mut self) -> DiziResult {
        self.player_stream_req().send(PlayerRequest::Stop)?;

        self.player_stream_res().recv()??;
        self.state.status = PlayerStatus::Stopped;
        Ok(())
    }

    pub fn toggle_play(&mut self) -> DiziResult<PlayerStatus> {
        match self.state.status {
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

    pub fn get_volume(&self) -> usize {
        self.state.volume
    }
    pub fn set_volume(&mut self, volume: usize) -> DiziResult {
        self.player_stream_req()
            .send(PlayerRequest::SetVolume(volume as f32 / 100.0))?;

        self.player_stream_res().recv()??;
        self.state.volume = volume;
        Ok(())
    }
    pub fn next_enabled(&self) -> bool {
        self.state.next
    }
    pub fn repeat_enabled(&self) -> bool {
        self.state.repeat
    }
    pub fn shuffle_enabled(&self) -> bool {
        self.state.shuffle
    }

    pub fn set_next(&mut self, next: bool) {
        self.state.next = next;
    }
    pub fn set_repeat(&mut self, repeat: bool) {
        self.state.repeat = repeat;
    }
    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.state.shuffle = shuffle;
        self.playlist_mut().file_playlist_mut().set_shuffle(shuffle);
        self.playlist_mut()
            .directory_playlist_mut()
            .set_shuffle(shuffle);
        if self.shuffle_enabled() {
            self.playlist_mut().file_playlist_mut().shuffle();
            self.playlist_mut().directory_playlist_mut().shuffle();
        } else {
            self.playlist_mut().file_playlist_mut().unshuffle();
            self.playlist_mut().directory_playlist_mut().shuffle();
        }
    }

    pub fn get_elapsed(&self) -> time::Duration {
        self.state.elapsed
    }
    pub fn set_elapsed(&mut self, elapsed: time::Duration) {
        self.state.elapsed = elapsed;
    }

    pub fn current_song_ref(&self) -> Option<&Song> {
        self.state.song.as_ref()
    }

    pub fn playlist_ref(&self) -> &PlaylistContext {
        &self.playlist_context
    }
    pub fn playlist_mut(&mut self) -> &mut PlaylistContext {
        &mut self.playlist_context
    }
}
