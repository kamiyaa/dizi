use std::fs;
use std::path::Path;

use log::{debug, log_enabled, Level};

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};
use dizi_lib::song::Song;

use crate::audio::{DiziPlaylist, PlayerFilePlaylist};
use crate::context::AppContext;
use crate::server::run_on_song_change;
use crate::util::mimetype::is_playable;

pub fn playlist_play(context: &mut AppContext, index: usize) -> DiziResult<()> {
    context.player_mut().play_from_playlist(index)?;
    run_on_song_change(context);
    Ok(())
}

pub fn playlist_load(context: &mut AppContext, cwd: &Path, path: &Path) -> DiziResult<()> {
    let shuffle_enabled = context.player_ref().shuffle_enabled();
    let playlist = context.player_mut().playlist_mut();
    if !playlist.file_playlist.is_empty() {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "playlist cannot be loaded because current playlist is not empty".to_string(),
        ));
    }

    let mut new_playlist = PlayerFilePlaylist::from_file(cwd, path)?;
    new_playlist.set_shuffle(shuffle_enabled);
    playlist.file_playlist = new_playlist;
    Ok(())
}

pub fn playlist_clear(context: &mut AppContext) -> DiziResult<()> {
    context.player_mut().playlist_mut().file_playlist.clear();
    Ok(())
}

pub fn playlist_append(context: &mut AppContext, path: &Path) -> DiziResult<Vec<Song>> {
    if path.is_dir() {
        let songs = recursively_find_songs(path);
        for song in songs.iter() {
            context
                .player_mut()
                .playlist_mut()
                .file_playlist_mut()
                .push(song.clone());
        }
        Ok(songs)
    } else if is_playable(path)? {
        let song = Song::new(path)?;
        context
            .player_mut()
            .playlist_mut()
            .file_playlist_mut()
            .push(song.clone());
        Ok(vec![song])
    } else {
        Ok(vec![])
    }
}

pub fn playlist_remove(context: &mut AppContext, index: usize) -> DiziResult<()> {
    let len = context.player_ref().playlist_ref().file_playlist.len();
    if index <= len {
        context
            .player_mut()
            .playlist_mut()
            .file_playlist
            .remove(index);
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

    let playlist = context.player_mut().playlist_mut().file_playlist_mut();
    if index >= playlist.len() {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "index out of range".to_string(),
        ));
    }

    playlist.swap(index, index - 1);

    Ok(())
}

pub fn playlist_move_down(context: &mut AppContext, index: usize) -> DiziResult<()> {
    let playlist = context.player_mut().playlist_mut().file_playlist_mut();

    if index + 1 >= playlist.len() {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "song is already at the end of playlist".to_string(),
        ));
    }

    playlist.swap(index, index + 1);

    Ok(())
}

fn recursively_find_songs(path: &Path) -> Vec<Song> {
    let mut songs: Vec<Song> = Vec::new();
    find_songs_rec(&mut songs, path);
    songs
}

fn find_songs_rec(songs: &mut Vec<Song>, path: &Path) {
    if let Ok(readdir) = fs::read_dir(path) {
        for entry in readdir.flatten() {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                find_songs_rec(songs, &entry_path);
            } else if let Ok(true) = is_playable(&entry_path) {
                if log_enabled!(Level::Debug) {
                    debug!("Adding {:?} to playlist", entry_path);
                }
                if let Ok(song) = Song::new(&entry_path) {
                    songs.push(song);
                }
            }
        }
    }
}
