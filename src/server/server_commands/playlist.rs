use std::path::Path;

use dizi_lib::error::DiziResult;
use dizi_lib::player::PlayerStatus;
use dizi_lib::song::Song;

use crate::context::AppContext;

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

pub fn playlist_play(context: &mut AppContext, index: usize) -> DiziResult<()> {
    context
        .player_context_mut()
        .player_mut()
        .play_from_playlist(index)?;
    Ok(())
}
