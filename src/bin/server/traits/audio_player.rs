use std::path::Path;
use std::time;

use dizi::error::DiziResult;
use dizi::player::{PlayerState, PlayerStatus};
use dizi::song::DiziAudioFile;

use crate::playlist::DiziPlaylist;

pub trait AudioPlayer {
    fn player_state(&self) -> PlayerState;

    fn play_directory(&mut self, path: &Path) -> DiziResult;
    fn play_from_playlist(&mut self, index: usize) -> DiziResult;

    fn play_again(&mut self) -> DiziResult;
    fn play_next(&mut self) -> DiziResult;
    fn play_previous(&mut self) -> DiziResult;

    fn pause(&mut self) -> DiziResult;
    fn resume(&mut self) -> DiziResult;
    fn stop(&mut self) -> DiziResult;
    fn toggle_play(&mut self) -> DiziResult<PlayerStatus>;

    fn fast_forward(&mut self, duration: time::Duration) -> DiziResult;
    fn rewind(&mut self, duration: time::Duration) -> DiziResult;

    fn get_volume(&self) -> usize;
    fn set_volume(&mut self, volume: usize) -> DiziResult;

    fn next_enabled(&self) -> bool;
    fn repeat_enabled(&self) -> bool;
    fn shuffle_enabled(&self) -> bool;

    fn set_next(&mut self, next: bool);
    fn set_repeat(&mut self, repeat: bool);
    fn set_shuffle(&mut self, shuffle: bool);

    fn get_elapsed(&self) -> time::Duration;
    fn set_elapsed(&mut self, elapsed: time::Duration);

    fn current_song_ref(&self) -> Option<&DiziAudioFile>;
    fn playlist_mut(&mut self) -> &mut DiziPlaylist;
}
