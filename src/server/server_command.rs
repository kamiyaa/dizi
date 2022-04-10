use dizi_lib::error::DiziResult;
use dizi_lib::player::PlayerStatus;
use dizi_lib::playlist::PlaylistType;
use dizi_lib::request::client::ClientRequest;
use dizi_lib::response::server::ServerBroadcastEvent;

use log::{debug, log_enabled, Level};

use crate::context::AppContext;
use crate::playlist::traits::{OrderedPlaylist, OrderedPlaylistEntry, ShufflePlaylist};
use crate::server_commands::*;

pub fn process_client_request(
    context: &mut AppContext,
    uuid: String,
    event: ClientRequest,
) -> DiziResult<()> {
    if log_enabled!(Level::Debug) {
        debug!("request: {:?}", event);
    }
    match event {
        ClientRequest::ServerQuit => {
            server::quit_server(context)?;
        }
        ClientRequest::ServerQuery { query } => {
            let res = server::query(context, &query)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::ServerQuery { query: res });
        }
        ClientRequest::ClientLeave { uuid } => {
            let index = context
                .events
                .server_broadcast_listeners
                .iter()
                .enumerate()
                .find(|(_, (listener_uuid, _))| listener_uuid == &uuid)
                .map(|(i, ..)| i);
            if let Some(index) = index {
                context.events.server_broadcast_listeners.remove(index);
            }
        }
        ClientRequest::PlayerState => {
            let state = context.player_ref().clone_player_state();
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerState { state });
        }
        ClientRequest::PlayerFilePlay { path: Some(p) } => {
            player_play(context, p.as_path())?;
            if let Some(song) = context.player_ref().current_song_ref() {
                let song = song.clone();
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlayerFilePlay { song });
            }
        }
        ClientRequest::PlayerPause => {
            player_pause(context)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerPause);
        }
        ClientRequest::PlayerResume => {
            player_resume(context)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerResume);
        }
        ClientRequest::PlayerVolumeUp { amount } => {
            let volume = player_volume_increase(context, amount)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerVolumeUpdate { volume });
        }
        ClientRequest::PlayerVolumeDown { amount } => {
            let volume = player_volume_decrease(context, amount)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerVolumeUpdate { volume });
        }
        ClientRequest::PlayerTogglePlay => {
            let status = player_toggle_play(context)?;
            match status {
                PlayerStatus::Playing => {
                    context
                        .events
                        .broadcast_event(ServerBroadcastEvent::PlayerResume);
                }
                PlayerStatus::Paused => {
                    context
                        .events
                        .broadcast_event(ServerBroadcastEvent::PlayerPause);
                }
                _ => {
                    context
                        .events
                        .broadcast_event(ServerBroadcastEvent::PlayerPause);
                }
            }
        }
        ClientRequest::PlayerPlayNext => {
            player_play_next(context)?;
            send_latest_song_info(context)?;
        }
        ClientRequest::PlayerPlayPrevious => {
            player_play_previous(context)?;
            send_latest_song_info(context);
        }
        ClientRequest::PlaylistAppend { path: Some(p) } => {
            let songs = playlist::playlist_append(context, &p)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistAppend { songs });
        }
        ClientRequest::PlaylistRemove { index: Some(index) } => {
            playlist::playlist_remove(context, index)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistRemove { index });
        }
        ClientRequest::PlaylistClear => {
            playlist::playlist_clear(context)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistClear);
        }
        ClientRequest::PlaylistMoveUp { index: Some(index) } => {
            playlist::playlist_move_up(context, index)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistSwapMove {
                    index1: index,
                    index2: index - 1,
                });
        }
        ClientRequest::PlaylistMoveDown { index: Some(index) } => {
            playlist::playlist_move_down(context, index)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistSwapMove {
                    index1: index,
                    index2: index + 1,
                });
        }
        ClientRequest::PlaylistPlay { index: Some(index) } => {
            playlist::playlist_play(context, index)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistPlay { index });
        }
        ClientRequest::PlaylistOpen {
            cwd: Some(cwd),
            path: Some(path),
        } => {
            playlist::playlist_load(context, &cwd, &path)?;
            let state = context.player_ref().clone_player_state();
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistOpen { state });
        }
        ClientRequest::PlayerToggleNext => {
            let enabled = context.player_ref().next_enabled();
            context.player_mut().set_next(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerNext { on: !enabled });
        }
        ClientRequest::PlayerToggleRepeat => {
            let enabled = context.player_ref().repeat_enabled();
            context.player_mut().set_repeat(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerRepeat { on: !enabled });
        }
        ClientRequest::PlayerToggleShuffle => {
            let enabled = context.player_ref().shuffle_enabled();
            context.player_mut().set_shuffle(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerShuffle { on: !enabled });
        }
        s => {
            if log_enabled!(Level::Debug) {
                debug!("'{:?}' not implemented", s);
            }
        }
    }
    Ok(())
}

pub fn send_latest_song_info(context: &mut AppContext) -> DiziResult<()> {
    match context.player_ref().playlist_ref().get_type() {
        PlaylistType::DirectoryListing => {
            if let Some(song) = context.player_ref().current_song_ref() {
                let song = song.clone();
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlayerFilePlay { song });
            }
        }
        PlaylistType::PlaylistFile => {
            let playlist = &context.player_ref().playlist_ref().file_playlist;

            if let Some(entry) = playlist.get_current_entry() {
                let index = entry.song_index;
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlaylistPlay { index });
            }
        }
    }
    Ok(())
}
