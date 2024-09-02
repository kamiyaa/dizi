use std::process::Command;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use uuid::Uuid;

use dizi::error::DiziResult;
use dizi::player::PlayerStatus;
use dizi::playlist::PlaylistType;
use dizi::request::client::ClientRequest;
use dizi::response::server::ServerBroadcastEvent;

use crate::client;
use crate::context::AppContext;
use crate::events::ServerEvent;
use crate::server_commands::*;
use crate::traits::{AudioPlayer, DiziPlaylistTrait};

pub fn process_server_event(context: &mut AppContext, event: ServerEvent) -> DiziResult {
    match event {
        ServerEvent::NewClient(stream) => {
            let client_tx2 = context.events.client_request_sender().clone();
            let (server_tx, server_rx) = mpsc::channel();

            let client_uuid = Uuid::new_v4();
            let uuid_string = client_uuid.to_string();
            thread::spawn(move || {
                client::handle_client(client_uuid, stream, client_tx2, server_rx)
            });
            context
                .events
                .add_broadcast_listener(uuid_string, server_tx);
        }
        ServerEvent::PlayerProgressUpdate(elapsed) => {
            context.player.set_elapsed(elapsed);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerProgressUpdate { elapsed });
        }
        ServerEvent::PlayerDone => {
            process_done_song(context)?;
        }
    }
    Ok(())
}

pub fn process_client_request(
    context: &mut AppContext,
    uuid: &str,
    event: ClientRequest,
) -> DiziResult {
    tracing::debug!("request: {:?} {:?}", uuid, event);
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
            let _ = context.events.server_broadcast_listeners.remove(&uuid);
        }
        ClientRequest::PlayerState => {
            let state = context.player.player_state();
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerState { state });
        }
        ClientRequest::PlayerFilePlay { path: Some(p) } => {
            player_play(context, p.as_path())?;
            if let Some(song) = context.player.current_song_ref() {
                let song = song.clone();
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlayerFilePlay { file: song });
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
            send_latest_song_info(context)?;
        }
        ClientRequest::PlaylistAppend { path: Some(p) } => {
            let songs = playlist::playlist_append(context, &p)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistAppend { audio_files: songs });
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
            let state = context.player.player_state();
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlaylistOpen { state });
        }
        ClientRequest::PlayerToggleNext => {
            let enabled = context.player.next_enabled();
            context.player.set_next(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerNext { on: !enabled });
        }
        ClientRequest::PlayerToggleRepeat => {
            let enabled = context.player.repeat_enabled();
            context.player.set_repeat(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerRepeat { on: !enabled });
        }
        ClientRequest::PlayerToggleShuffle => {
            let enabled = context.player.shuffle_enabled();
            context.player.set_shuffle(!enabled);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerShuffle { on: !enabled });
        }
        ClientRequest::PlayerFastForward { amount } => {
            let duration = Duration::from_secs(amount as u64);
            context.player.fast_forward(duration)?;
        }
        ClientRequest::PlayerRewind { amount } => {
            let duration = Duration::from_secs(amount as u64);
            context.player.rewind(duration)?;
        }
        ClientRequest::ServerQueryAll => {}
        s => {
            tracing::debug!("'{:?}' not implemented", s);
        }
    }
    Ok(())
}

pub fn send_latest_song_info(context: &mut AppContext) -> DiziResult {
    match context.player.playlist.playlist_type {
        PlaylistType::DirectoryListing => {
            if let Some(file) = context.player.current_song_ref() {
                let file = file.clone();
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlayerFilePlay { file });
            }
        }
        PlaylistType::PlaylistFile => {
            if let Some(order_index) = context.player.playlist.order_index {
                let entry_index = context.player.playlist.order[order_index];
                context
                    .events
                    .broadcast_event(ServerBroadcastEvent::PlaylistPlay { index: entry_index });
            }
        }
    }
    Ok(())
}

pub fn process_done_song(context: &mut AppContext) -> DiziResult {
    tracing::debug!("Processing done song trigger");

    let next_enabled = context.player.next_enabled();
    let repeat_enabled = context.player.repeat_enabled();

    if next_enabled {
        if !repeat_enabled && end_of_playlist(context) {
            context.player.stop()?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerStop);
        } else {
            player_play_next(context)?;
            send_latest_song_info(context)?;
        }
    } else if repeat_enabled {
        player_play_again(context)?;
        send_latest_song_info(context)?;
    } else {
    }

    Ok(())
}

pub fn end_of_playlist(context: &AppContext) -> bool {
    context.player.playlist.is_end()
}

pub fn run_on_song_change(context: &AppContext) {
    let server_config = context.config_ref().server_ref();
    if let Some(path) = server_config.on_song_change.as_ref() {
        let on_song_change_script = path.to_path_buf();
        thread::spawn(move || {
            if let Ok(mut child) = Command::new(on_song_change_script).spawn() {
                let _ = child.wait();
            }
        });
    }
}
