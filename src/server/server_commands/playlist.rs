use std::path::Path;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};
use dizi_lib::player::PlayerStatus;
use dizi_lib::song::Song;

use crate::audio::read_playlist;
use crate::context::AppContext;

pub fn playlist_play(context: &mut AppContext, index: usize) -> DiziResult<()> {
    context
        .player_context_mut()
        .player_mut()
        .play_from_playlist(index)?;
    Ok(())
}

pub fn playlist_load(context: &mut AppContext, cwd: &Path, path: &Path) -> DiziResult<()> {
    let mut playlist = context.player_context_mut().player_mut().playlist_mut();
    if !playlist.is_empty() {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "playlist cannot be loaded because current playlist is not empty".to_string(),
        ));
    }

    let new_playlist = read_playlist(cwd, path)?;
    *playlist = new_playlist;
    Ok(())
}

pub fn playlist_clear(context: &mut AppContext) -> DiziResult<()> {
    let mut playlist = context.player_context_mut().player_mut().playlist_mut();
    playlist.clear();
    Ok(())
}

pub fn playlist_append(context: &mut AppContext, path: &Path) -> DiziResult<Song> {
    let song = Song::new(path)?;
    context
        .player_context_mut()
        .player_mut()
        .playlist_mut()
        .append_song(song.clone());
    Ok(song)
}

pub fn playlist_remove(context: &mut AppContext, index: usize) -> DiziResult<()> {
    let len = context
        .player_context_ref()
        .player_ref()
        .playlist_ref()
        .len();
    if index <= len {
        context
            .player_context_mut()
            .player_mut()
            .playlist_mut()
            .remove_song(index);
    }
    Ok(())
}

pub fn playlist_move_up(context: &mut AppContext, index: usize) -> DiziResult<()> {
    if index == 0 {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "song is already at the start of playlist".to_string(),
        ));
    }

    let mut playlist = context.player_context_mut().player_mut().playlist_mut();

    if index >= playlist.len() {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "index out of range".to_string(),
        ));
    }

    playlist.list_mut().swap(index, index - 1);

    Ok(())
}

pub fn playlist_move_down(context: &mut AppContext, index: usize) -> DiziResult<()> {
    let playlist = context.player_context_mut().player_mut().playlist_mut();

    if index + 1 >= playlist.len() {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "song is already at the end of playlist".to_string(),
        ));
    }

    playlist.list_mut().swap(index, index + 1);

    Ok(())
}
