use dizi_lib::error::DiziResult;
use dizi_lib::playlist::PlaylistType;

use crate::commands::change_directory;
use crate::commands::cursor_move::set_playlist_index;
use crate::commands::search_string;
use crate::config::option::WidgetType;
use crate::context::AppContext;

fn _directory_goto_playing(context: &mut AppContext) -> DiziResult {
    let player_state = context.server_state_ref().player_ref();

    if let Some(song) = player_state.song.clone() {
        let file_path = song.file_path();
        if let Some(parent) = file_path.parent() {
            change_directory::change_directory(context, parent)?;
        }
        let file_name = song.file_name();
        search_string::search_exact(context, file_name)?;
    }
    Ok(())
}

fn _playlist_goto_playing(context: &mut AppContext) -> DiziResult {
    let player_state = context.server_state_ref().player_ref();

    match player_state.playlist_status {
        PlaylistType::DirectoryListing => {}
        PlaylistType::PlaylistFile => {
            if let Some(index) = player_state.playlist.playing_index {
                set_playlist_index(context, index);
            }
        }
    }
    Ok(())
}

pub fn goto_playing(context: &mut AppContext) -> DiziResult {
    let widget = context.get_view_widget();
    match widget {
        WidgetType::FileBrowser => _directory_goto_playing(context)?,
        WidgetType::Playlist => _playlist_goto_playing(context)?,
        _ => {}
    }
    Ok(())
}
