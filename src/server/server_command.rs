use dizi_lib::error::DiziResult;
use dizi_lib::player::PlayerStatus;
use dizi_lib::playlist::PlaylistStatus;
use dizi_lib::request::client::ClientRequest;
use dizi_lib::response::server::ServerBroadcastEvent;
use dizi_lib::song::Song;

use crate::context::AppContext;
use crate::server_commands::*;

pub fn process_client_request(context: &mut AppContext, event: ClientRequest) -> DiziResult<()> {
    eprintln!("request: {:?}", event);
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
            let state = context
                .player_context_ref()
                .player_ref()
                .clone_player_state();
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerState { state });
        }
        ClientRequest::PlayerFilePlay { path: Some(p) } => {
            let song = Song::new(p.as_path())?;
            player_play(context, song.file_path())?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerFilePlay { song });
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
        ClientRequest::PlayerGetVolume => {
            eprintln!(
                "Error: '{:?}' not implemented",
                ClientRequest::PlayerGetVolume
            );
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
            let len1 = context
                .player_context_ref()
                .player_ref()
                .dirlist_playlist_ref()
                .len();

            let len2 = context
                .player_context_ref()
                .player_ref()
                .playlist_ref()
                .len();

            let len = if len1 < len2 { len2 } else { len1 };
            for i in (1..len) {
                if player_play_next(context, i).is_err() {
                    continue;
                }
                send_latest_song_info(context)?;
                break;
            }
        }
        ClientRequest::PlayerPlayPrevious => {
            player_play_previous(context)?;
            send_latest_song_info(context);
        }
        ClientRequest::PlaylistAppend { path: Some(p) } => {
            let song = playlist::playlist_append(context, &p)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistAppend { song });
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
            let state = context
                .player_context_ref()
                .player_ref()
                .clone_player_state();
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerState { state });
        }
        ClientRequest::PlayerToggleNext => {
            let enabled = context.player_context_ref().player_ref().next_enabled();
            context.player_context_mut().player_mut().set_next(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerNext { on: !enabled });
        }
        ClientRequest::PlayerToggleRepeat => {
            let enabled = context.player_context_ref().player_ref().repeat_enabled();
            context
                .player_context_mut()
                .player_mut()
                .set_repeat(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerRepeat { on: !enabled });
        }
        ClientRequest::PlayerToggleShuffle => {
            let enabled = context.player_context_ref().player_ref().shuffle_enabled();
            context
                .player_context_mut()
                .player_mut()
                .set_shuffle(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerShuffle { on: !enabled });
        }
        s => {
            eprintln!("Error: '{:?}' not implemented", s);
        }
    }
    Ok(())
}

pub fn send_latest_song_info(context: &mut AppContext) -> DiziResult<()> {
    match context.player_context_ref().player_ref().playlist_status() {
        PlaylistStatus::DirectoryListing => {
            if let Some(song) = context.player_context_ref().player_ref().current_song_ref() {
                let song = song.clone();
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlayerFilePlay { song });
            }
        }
        PlaylistStatus::PlaylistFile => {
            if let Some(index) = context
                .player_context_ref()
                .player_ref()
                .playlist_ref()
                .get_playing_index()
            {
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlaylistPlay { index });
            }
        }
    }
    Ok(())
}
