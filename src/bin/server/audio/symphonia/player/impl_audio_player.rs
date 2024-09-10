use std::path;
use std::time;

use dizi::error::{DiziError, DiziErrorKind, DiziResult};
use dizi::player::{PlayerState, PlayerStatus};
use dizi::playlist::PlaylistType;
use dizi::song::DiziAudioFile;
use dizi::song::DiziSongEntry;

use crate::audio::request::PlayerRequest;
use crate::context::PlaylistContext;
use crate::playlist::DiziPlaylist;
use crate::traits::{AudioPlayer, DiziPlaylistTrait};
use crate::util::mimetype::{get_mimetype, is_mimetype_audio, is_mimetype_video};

use super::SymphoniaPlayer;

impl AudioPlayer for SymphoniaPlayer {
    fn player_state(&self) -> PlayerState {
        let mut state = self.state.clone();
        state.playlist = self.playlist_context.file_playlist.to_file_playlist();
        state.playlist_status = self.playlist_context.current_playlist_type;
        state
    }

    fn play_directory(&mut self, path: &path::Path) -> DiziResult {
        let mimetype = get_mimetype(path)?;
        if !is_mimetype_audio(&mimetype) && !is_mimetype_video(&mimetype) {
            return Err(DiziError::new(
                DiziErrorKind::NotAudioFile,
                format!("File mimetype is not of type audio: '{}'", mimetype),
            ));
        }

        let shuffle_enabled = self.shuffle_enabled();
        if let Some(parent) = path.parent() {
            let mut playlist = DiziPlaylist::from_dir(parent)?;
            // find the song we're playing in the playlist and set playing index
            // equal to the playing song
            let index = playlist
                .contents
                .iter()
                .enumerate()
                .find(|(_, p)| p.file_path() == path)
                .map(|(i, _)| i);

            if shuffle_enabled {
                playlist.unshuffle();
            }
            playlist.order_index = index;
            if shuffle_enabled {
                playlist.shuffle();
            }

            playlist.load_current_entry_metadata()?;
            if let Some(entry) = playlist.current_entry() {
                if let DiziSongEntry::Loaded(audio_file) = entry.entry {
                    self.play(&audio_file)?;
                }
            }

            self.playlist_context.directory_playlist = playlist;
            self.set_playlist_type(PlaylistType::DirectoryListing);
        }
        Ok(())
    }

    fn play_from_playlist(&mut self, index: usize) -> DiziResult {
        let shuffle_enabled = self.shuffle_enabled();
        let playlist = &mut self.playlist_context.file_playlist;

        if shuffle_enabled {
            playlist.unshuffle();
        }
        // unshuffle the playlist before choosing setting the new index
        playlist.order_index = Some(index);
        // reshuffle playlist upon playing new file
        if shuffle_enabled {
            playlist.shuffle();
        }

        playlist.load_current_entry_metadata()?;
        if let Some(entry) = playlist.current_entry() {
            if let DiziSongEntry::Loaded(audio_file) = entry.entry {
                self.play(&audio_file)?;
            }
        }
        self.set_playlist_type(PlaylistType::PlaylistFile);

        Ok(())
    }

    fn play_again(&mut self) -> DiziResult {
        let playlist = self.playlist_context.current_playlist_ref();

        if let Some(entry) = playlist.current_entry() {
            if let DiziSongEntry::Loaded(audio_file) = entry.entry {
                self.play(&audio_file)?;
            }
        }
        Ok(())
    }

    fn play_next(&mut self) -> DiziResult {
        let playlist = self.playlist_context.current_playlist_mut();

        let song_entry = playlist.next_song_peak().ok_or_else(|| {
            DiziError::new(DiziErrorKind::ParseError, "Playlist error".to_string())
        })?;
        playlist.order_index = Some(song_entry.order_index);

        playlist.load_current_entry_metadata()?;
        if let Some(entry) = playlist.current_entry() {
            if let DiziSongEntry::Loaded(audio_file) = entry.entry {
                self.play(&audio_file)?;
            }
        }
        Ok(())
    }

    fn play_previous(&mut self) -> DiziResult {
        let playlist = self.playlist_context.current_playlist_mut();

        let song_entry = playlist.previous_song_peak().ok_or_else(|| {
            DiziError::new(DiziErrorKind::ParseError, "Playlist error".to_string())
        })?;
        playlist.order_index = Some(song_entry.order_index);

        playlist.load_current_entry_metadata()?;
        if let Some(entry) = playlist.current_entry() {
            if let DiziSongEntry::Loaded(audio_file) = entry.entry {
                self.play(&audio_file)?;
            }
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
    fn fast_forward(&mut self, offset: time::Duration) -> DiziResult {
        self.player_stream_req()
            .send(PlayerRequest::FastForward { offset })?;
        Ok(())
    }
    fn rewind(&mut self, offset: time::Duration) -> DiziResult {
        self.player_stream_req()
            .send(PlayerRequest::Rewind { offset })?;
        Ok(())
    }

    fn get_volume(&self) -> usize {
        self.state.volume
    }
    fn set_volume(&mut self, volume: usize) -> DiziResult {
        self.player_stream_req().send(PlayerRequest::SetVolume {
            volume: volume as f32 / 100.0,
        })?;

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

        if self.shuffle_enabled() {
            self.playlist_context.current_playlist_mut().shuffle();
        } else {
            self.playlist_context.current_playlist_mut().unshuffle();
        }
    }

    fn set_elapsed(&mut self, elapsed: time::Duration) {
        self.state.elapsed = elapsed;
    }

    fn current_song_ref(&self) -> Option<&DiziAudioFile> {
        self.state.song.as_ref()
    }
    fn playlist_context_mut(&mut self) -> &mut PlaylistContext {
        &mut self.playlist_context
    }
}
