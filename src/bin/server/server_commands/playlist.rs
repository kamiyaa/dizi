use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};

use dizi::error::{DiziError, DiziErrorKind, DiziResult};
use dizi::song::{DiziAudioFile, DiziFile, DiziSongEntry};

use crate::context::AppContext;
use crate::playlist::DiziPlaylist;
use crate::server_util::run_on_song_change;
use crate::traits::{AudioPlayer, DiziPlaylistTrait};
use crate::util::mimetype::is_playable;

pub fn playlist_play(context: &mut AppContext, index: usize) -> DiziResult {
    context.player.play_from_playlist(index)?;
    run_on_song_change(context);
    Ok(())
}

pub fn playlist_load(context: &mut AppContext, cwd: &Path, path: &Path) -> DiziResult {
    let shuffle_enabled = context.player.shuffle_enabled();
    if !context
        .player
        .playlist_context_mut()
        .file_playlist
        .is_empty()
    {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "playlist cannot be loaded because current playlist is not empty".to_string(),
        ));
    }

    let mut new_playlist = DiziPlaylist::from_file(cwd, path)?;
    if shuffle_enabled {
        new_playlist.shuffle();
    }
    context.player.playlist_context.file_playlist = new_playlist;
    Ok(())
}

pub fn playlist_clear(context: &mut AppContext) -> DiziResult {
    context
        .player
        .playlist_context_mut()
        .current_playlist_mut()
        .clear();
    Ok(())
}

pub fn playlist_append(context: &mut AppContext, path: &Path) -> DiziResult<Vec<DiziAudioFile>> {
    if path.is_dir() {
        let audio_files = recursively_find_songs(path);
        for audio_file in audio_files.iter() {
            let entry = DiziSongEntry::Loaded(audio_file.clone());
            context
                .player
                .playlist_context_mut()
                .current_playlist_mut()
                .push(entry);
        }
        Ok(audio_files)
    } else if is_playable(path)? {
        let file = DiziFile::new(path);
        let audio_file = DiziAudioFile::try_from(file)?;
        let entry = DiziSongEntry::Loaded(audio_file.clone());
        context
            .player
            .playlist_context_mut()
            .current_playlist_mut()
            .push(entry);
        Ok(vec![audio_file])
    } else {
        Ok(vec![])
    }
}

pub fn playlist_remove(context: &mut AppContext, index: usize) -> DiziResult {
    let len = context.player.playlist_context.current_playlist_ref().len();
    if index <= len {
        context
            .player
            .playlist_context
            .current_playlist_mut()
            .remove(index);
    }
    Ok(())
}

pub fn playlist_move_up(context: &mut AppContext, index: usize) -> DiziResult {
    if index == 0 {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "song is already at the start of playlist".to_string(),
        ));
    }

    let playlist = &mut context.player.playlist_context_mut().file_playlist;
    if index >= playlist.len() {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "index out of range".to_string(),
        ));
    }

    playlist.swap(index, index - 1);

    Ok(())
}

pub fn playlist_move_down(context: &mut AppContext, index: usize) -> DiziResult {
    let playlist = &mut context.player.playlist_context_mut().file_playlist;

    if index + 1 >= playlist.len() {
        return Err(DiziError::new(
            DiziErrorKind::InvalidParameters,
            "song is already at the end of playlist".to_string(),
        ));
    }

    playlist.swap(index, index + 1);

    Ok(())
}

fn sort_function(p1: &Path, p2: &Path) -> Ordering {
    let p1_is_dir = p1.is_dir();
    let p2_is_dir = p2.is_dir();
    match (p1_is_dir, p2_is_dir) {
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
        _ => alphanumeric_sort::compare_path(p1, p2),
    }
}

fn recursively_find_songs(path: &Path) -> Vec<DiziAudioFile> {
    let mut songs: Vec<_> = Vec::new();
    find_songs_rec(&mut songs, path);
    songs
}

fn find_songs_rec(songs: &mut Vec<DiziAudioFile>, path: &Path) {
    if let Ok(readdir) = fs::read_dir(path) {
        let mut paths: Vec<PathBuf> = readdir.flatten().map(|entry| entry.path()).collect();
        paths.sort_by(|p1, p2| sort_function(p1, p2));
        for entry_path in paths.iter() {
            if entry_path.is_dir() {
                find_songs_rec(songs, entry_path);
                continue;
            }

            if let Ok(true) = is_playable(entry_path) {
                tracing::debug!("Adding {:?} to playlist", entry_path);
                let file = DiziFile::new(entry_path);
                if let Ok(audio_file) = DiziAudioFile::try_from(file) {
                    songs.push(audio_file);
                }
            }
        }
    }
}
