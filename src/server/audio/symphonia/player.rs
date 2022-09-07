use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time;

use cpal::traits::HostTrait;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};
use dizi_lib::player::{PlayerState, PlayerStatus};
use dizi_lib::playlist::PlaylistType;
use dizi_lib::song::Song;

use crate::audio::device::get_default_host;
use crate::audio::request::PlayerRequest;
use crate::audio::symphonia::stream::PlayerStream;
use crate::audio::traits::AudioPlayer;
use crate::config;
use crate::context::PlaylistContext;
use crate::events::ServerEventSender;
use crate::playlist::playlist_directory::PlaylistDirectory;
use crate::playlist::playlist_file::PlaylistFile;
use crate::playlist::traits::{OrderedPlaylist, ShufflePlaylist};
use crate::util::mimetype::{get_mimetype, is_mimetype_audio, is_mimetype_video};

#[derive(Debug)]
pub struct SymphoniaPlayer {
    state: PlayerState,

    playlist_context: PlaylistContext,

    player_req_tx: mpsc::Sender<PlayerRequest>,
    player_res_rx: mpsc::Receiver<DiziResult>,
}

impl SymphoniaPlayer {
    pub fn new(config_t: &config::AppConfig, event_tx: ServerEventSender) -> Self {
        let audio_host = get_default_host(config_t.server_ref().audio_system);
        let audio_device = audio_host.default_output_device().unwrap();

        let (player_req_tx, player_req_rx) = mpsc::channel();
        let (player_res_tx, player_res_rx) = mpsc::channel();

        let _: JoinHandle<DiziResult> = thread::spawn(move || {
            let mut stream =
                PlayerStream::new(event_tx, player_res_tx, player_req_rx, audio_device);
            stream.listen_for_events()?;
            Ok(())
        });

        let server_config = config_t.server_ref();
        let player_config = server_config.player_ref();

        let mut playlist_context = PlaylistContext::default();
        playlist_context.file_playlist =
            PlaylistFile::from_file(&PathBuf::from("/"), server_config.playlist_ref())
                .unwrap_or_else(|_| PlaylistFile::new(Vec::new()));

        let state = PlayerState {
            next: player_config.next,
            repeat: player_config.repeat,
            shuffle: player_config.shuffle,
            volume: config_t.server_ref().player_ref().volume,
            audio_host: audio_host.id().name().to_lowercase(),
            ..PlayerState::default()
        };

        Self {
            state,
            playlist_context,
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

    fn play(&mut self, song: &Song) -> DiziResult {
        self.player_stream_req()
            .send(PlayerRequest::Play(song.clone()))?;
        self.set_volume(self.get_volume())?;
        let _resp = self.player_stream_res().recv()??;

        self.state.status = PlayerStatus::Playing;
        self.state.song = Some(song.clone());
        Ok(())
    }
}

impl AudioPlayer for SymphoniaPlayer {
    fn player_state(&self) -> PlayerState {
        let mut state = self.state.clone();
        state.playlist = self.playlist_ref().file_playlist.clone_file_playlist();
        state
    }

    fn play_directory(&mut self, path: &Path) -> DiziResult {
        let mimetype = get_mimetype(path)?;
        if !is_mimetype_audio(&mimetype) && !is_mimetype_video(&mimetype) {
            return Err(DiziError::new(
                DiziErrorKind::NotAudioFile,
                format!("File mimetype is not of type audio: '{}'", mimetype),
            ));
        }

        let shuffle_enabled = self.shuffle_enabled();
        if let Some(parent) = path.parent() {
            let mut playlist = PlaylistDirectory::from_path(parent)?;
            // find the song we're playing in the playlist and set playing index
            // equal to the playing song
            let index = playlist
                .iter()
                .enumerate()
                .find(|(_, p)| p.file_path() == path)
                .map(|(i, _)| i);

            playlist.set_playlist_index(index);
            playlist.set_shuffle(shuffle_enabled);
            if playlist.shuffle_enabled() {
                playlist.shuffle();
            }
            if let Some(entry) = playlist.current_entry_details() {
                self.play(&entry.entry)?;
            }
            self.playlist_context.directory_playlist = playlist;
            self.state.playlist_status = PlaylistType::DirectoryListing;
            self.playlist_context
                .set_type(PlaylistType::DirectoryListing);
        }
        Ok(())
    }

    fn play_from_playlist(&mut self, index: usize) -> DiziResult {
        let shuffle_enabled = self.shuffle_enabled();
        let playlist = self.playlist_mut().file_playlist_mut();

        // unshuffle the playlist before choosing setting the new index
        playlist.unshuffle();
        playlist.set_playlist_index(Some(index));
        playlist.set_shuffle(shuffle_enabled);
        if playlist.shuffle_enabled() {
            playlist.shuffle();
        }
        if let Some(entry) = playlist.current_entry_details() {
            self.play(&entry.entry)?;
            self.state.playlist_status = PlaylistType::PlaylistFile;
            self.playlist_context.set_type(PlaylistType::PlaylistFile);
        }
        Ok(())
    }

    fn play_again(&mut self) -> DiziResult {
        if let Some(entry) = self.playlist_ref().current_entry_details() {
            self.play(&entry.entry)?;
        }
        Ok(())
    }

    fn play_next(&mut self) -> DiziResult {
        if let Some(entry) = self.playlist_mut().next_song() {
            self.play(&entry.entry)?;
        }
        Ok(())
    }

    fn play_previous(&mut self) -> DiziResult {
        if let Some(entry) = self.playlist_mut().previous_song() {
            self.play(&entry.entry)?;
        }
        Ok(())
    }

    fn pause(&mut self) -> DiziResult {
        self.player_stream_req().send(PlayerRequest::Pause)?;

        self.player_stream_res().recv()??;
        self.state.status = PlayerStatus::Paused;
        Ok(())
    }

    fn resume(&mut self) -> DiziResult {
        self.player_stream_req().send(PlayerRequest::Resume)?;

        self.player_stream_res().recv()??;
        self.state.status = PlayerStatus::Playing;
        Ok(())
    }

    fn stop(&mut self) -> DiziResult {
        self.player_stream_req().send(PlayerRequest::Stop)?;

        self.player_stream_res().recv()??;
        self.state.status = PlayerStatus::Stopped;
        Ok(())
    }

    fn toggle_play(&mut self) -> DiziResult<PlayerStatus> {
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

    fn get_volume(&self) -> usize {
        self.state.volume
    }
    fn set_volume(&mut self, volume: usize) -> DiziResult {
        self.player_stream_req()
            .send(PlayerRequest::SetVolume(volume as f32 / 100.0))?;

        self.player_stream_res().recv()??;
        self.state.volume = volume;
        Ok(())
    }
    fn next_enabled(&self) -> bool {
        self.state.next
    }
    fn repeat_enabled(&self) -> bool {
        self.state.repeat
    }
    fn shuffle_enabled(&self) -> bool {
        self.state.shuffle
    }

    fn set_next(&mut self, next: bool) {
        self.state.next = next;
    }
    fn set_repeat(&mut self, repeat: bool) {
        self.state.repeat = repeat;
    }
    fn set_shuffle(&mut self, shuffle: bool) {
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

    fn get_elapsed(&self) -> time::Duration {
        self.state.elapsed
    }
    fn set_elapsed(&mut self, elapsed: time::Duration) {
        self.state.elapsed = elapsed;
    }

    fn current_song_ref(&self) -> Option<&Song> {
        self.state.song.as_ref()
    }

    fn playlist_ref(&self) -> &PlaylistContext {
        &self.playlist_context
    }
    fn playlist_mut(&mut self) -> &mut PlaylistContext {
        &mut self.playlist_context
    }
}
