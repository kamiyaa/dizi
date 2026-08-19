use std::path::Path;
use std::time;

use dizi::error::AppResult;
use dizi::player::{PlayerState, PlayerStatus};
use dizi::song::DiziAudioFile;

use crate::context::PlaylistContext;

pub trait AudioPlayer {
    fn player_state(&self) -> PlayerState;

    fn play_directory(&mut self, path: &Path) -> AppResult;
    fn play_from_playlist(&mut self, index: usize) -> AppResult;

    fn play_again(&mut self) -> AppResult;
    fn play_next(&mut self) -> AppResult;
    fn play_previous(&mut self) -> AppResult;

    fn pause(&mut self) -> AppResult;
    fn resume(&mut self) -> AppResult;
    fn stop(&mut self) -> AppResult;
    fn toggle_play(&mut self) -> AppResult<PlayerStatus>;

    fn fast_forward(&mut self, duration: time::Duration) -> AppResult;
    fn rewind(&mut self, duration: time::Duration) -> AppResult;

    fn get_volume(&self) -> usize;
    fn set_volume(&mut self, volume: usize) -> AppResult;

    fn next_enabled(&self) -> bool;
    fn repeat_enabled(&self) -> bool;
    fn shuffle_enabled(&self) -> bool;

    fn set_next(&mut self, next: bool);
    fn set_repeat(&mut self, repeat: bool);
    fn set_shuffle(&mut self, shuffle: bool);

    fn set_elapsed(&mut self, elapsed: time::Duration);

    fn current_song_ref(&self) -> Option<&DiziAudioFile>;

    fn playlist_context_mut(&mut self) -> &mut PlaylistContext;
}
