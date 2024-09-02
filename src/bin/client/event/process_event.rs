use std::io;
use std::path;

use dizi::song::DiziSongEntry;
use signal_hook::consts::signal;
use termion::event::{Event, Key};

use dizi::error::DiziResult;
use dizi::player::PlayerStatus;
use dizi::playlist::PlaylistType;
use dizi::response::server::ServerBroadcastEvent;

use crate::config::option::WidgetType;
use crate::config::KeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::fs::JoshutoDirList;
use crate::key_command::{Command, CommandKeybind};
use crate::ui;
use crate::ui::views::TuiCommandMenu;

pub fn get_input_while_composite<'a>(
    backend: &mut ui::AppBackend,
    context: &mut AppContext,
    keymap: &'a KeyMapping,
) -> Option<&'a Command> {
    let mut keymap = keymap;

    context.flush_event();

    loop {
        backend.render(TuiCommandMenu::new(context, keymap));

        if let Ok(event) = context.poll_event() {
            match event {
                AppEvent::Termion(event) => {
                    match event {
                        Event::Key(Key::Esc) => return None,
                        event => match keymap.get(&event) {
                            Some(CommandKeybind::SimpleKeybind(s)) => {
                                return Some(s);
                            }
                            Some(CommandKeybind::CompositeKeybind(m)) => {
                                keymap = m;
                            }
                            None => return None,
                        },
                    }
                    context.flush_event();
                }
                event => process_noninteractive(event, context),
            }
        }
    }
}

pub fn process_server_event(context: &mut AppContext, s: &str) -> DiziResult {
    let server_broadcast_event: ServerBroadcastEvent = serde_json::from_str(s)?;

    match server_broadcast_event {
        ServerBroadcastEvent::ServerQuit => {
            context.quit = QuitType::Server;
        }
        ServerBroadcastEvent::ServerError { msg } => {
            context
                .message_queue_mut()
                .push_error(format!("Server: {}", msg));
        }
        ServerBroadcastEvent::ServerQuery { .. } => {}
        ServerBroadcastEvent::ServerQueryAll { .. } => {}
        ServerBroadcastEvent::PlayerState { mut state } => {
            if !state.playlist.is_empty() {
                let old_state = &context.server_state_ref().player;

                let playlist_len = state.playlist.len();
                let new_cursor_index = old_state
                    .playlist
                    .get_cursor_index()
                    .map(|s| {
                        if s < playlist_len {
                            s
                        } else {
                            playlist_len - 1
                        }
                    })
                    .unwrap_or_else(|| 0);
                state.playlist.set_cursor_index(Some(new_cursor_index));
            }
            context.server_state_mut().player = state;
        }
        ServerBroadcastEvent::PlaylistOpen { mut state } => {
            if !state.playlist.is_empty() {
                let old_state = &context.server_state_ref().player;

                let playlist_len = state.playlist.len();
                let new_cursor_index = old_state
                    .playlist
                    .get_cursor_index()
                    .map(|s| {
                        if s < playlist_len {
                            s
                        } else {
                            playlist_len - 1
                        }
                    })
                    .unwrap_or_else(|| 0);
                state.playlist.set_cursor_index(Some(new_cursor_index));
            }
            context.server_state_mut().player = state;
            let len = context.server_state_ref().player.playlist.len();
            context
                .message_queue_mut()
                .push_success(format!("Loaded {} songs to playlist", len));
        }
        ServerBroadcastEvent::PlayerFilePlay { file: song } => {
            context.server_state_mut().player.song = Some(song);
            context.server_state_mut().player.status = PlayerStatus::Playing;
            context.server_state_mut().player.playlist_status = PlaylistType::DirectoryListing;
        }
        ServerBroadcastEvent::PlayerPause => {
            context.server_state_mut().player.status = PlayerStatus::Paused;
        }
        ServerBroadcastEvent::PlayerResume => {
            context.server_state_mut().player.status = PlayerStatus::Playing;
        }
        ServerBroadcastEvent::PlayerStop => {
            context.server_state_mut().player.status = PlayerStatus::Stopped;
        }
        ServerBroadcastEvent::PlayerShuffle { on } => {
            context.server_state_mut().player.shuffle = on;
            let setting = "Shuffle";
            let status = if on { "ON" } else { "OFF" };
            context
                .message_queue_mut()
                .push_success(format!("{} {}", setting, status));
        }
        ServerBroadcastEvent::PlayerRepeat { on } => {
            context.server_state_mut().player.repeat = on;
            let setting = "Repeat";
            let status = if on { "ON" } else { "OFF" };
            context
                .message_queue_mut()
                .push_success(format!("{} {}", setting, status));
        }
        ServerBroadcastEvent::PlayerNext { on } => {
            context.server_state_mut().player.next = on;
            let setting = "Next";
            let status = if on { "ON" } else { "OFF" };
            context
                .message_queue_mut()
                .push_success(format!("{} {}", setting, status));
        }
        ServerBroadcastEvent::PlayerVolumeUpdate { volume } => {
            context.server_state_mut().player.volume = volume;
        }
        ServerBroadcastEvent::PlayerProgressUpdate { elapsed } => {
            context.server_state_mut().player.elapsed = elapsed;
        }
        ServerBroadcastEvent::PlaylistSwapMove { index1, index2 } => {
            let playlist = &mut context.server_state_mut().player.playlist;
            playlist.list_mut().swap(index1, index2);
            playlist.set_cursor_index(Some(index2));
            if let Some(playing_index) = playlist.get_playing_index() {
                if playing_index == index1 {
                    playlist.set_playing_index(Some(index2));
                }
                if playing_index == index2 {
                    playlist.set_playing_index(Some(index1));
                }
            }
        }
        ServerBroadcastEvent::PlaylistClear => {
            let playlist_len = context.server_state_mut().player.playlist.len();
            context.server_state_mut().player.playlist.clear();
            context.set_view_widget(WidgetType::FileBrowser);
            context
                .message_queue_mut()
                .push_success(format!("Removed {playlist_len} songs from playlist"));
        }
        ServerBroadcastEvent::PlaylistAppend { audio_files } => {
            let len = audio_files.len();
            let entries: Vec<_> = audio_files
                .into_iter()
                .map(|s| DiziSongEntry::Loaded(s))
                .collect();
            context
                .server_state_mut()
                .player
                .playlist
                .list_mut()
                .extend_from_slice(&entries);
            if context
                .server_state_ref()
                .player
                .playlist
                .get_cursor_index()
                .is_none()
            {
                context
                    .server_state_mut()
                    .player
                    .playlist
                    .set_cursor_index(Some(0));
            }
            context
                .message_queue_mut()
                .push_success(format!("Added {len} songs to playlist"));
        }
        ServerBroadcastEvent::PlaylistRemove { index } => {
            context
                .server_state_mut()
                .player
                .playlist
                .remove_song(index);
        }
        ServerBroadcastEvent::PlaylistPlay { index } => {
            let len = context.server_state_ref().player.playlist.len();
            if index < len {
                let player = &mut context.server_state_mut().player;
                player.status = PlayerStatus::Playing;
                player.playlist_status = PlaylistType::PlaylistFile;
                let cursor_index = player.playlist.get_cursor_index();
                let playing_index = player.playlist.get_playing_index();
                if playing_index == cursor_index {
                    player.playlist.set_cursor_index(Some(index));
                }
                player.playlist.set_playing_index(Some(index));
            }
        }
    }
    Ok(())
}

pub fn process_noninteractive(event: AppEvent, context: &mut AppContext) {
    match event {
        AppEvent::PreviewDir { path, res } => process_dir_preview(context, path, *res),
        AppEvent::Signal(signal::SIGWINCH) => {}
        _ => {}
    }
}

pub fn process_dir_preview(
    context: &mut AppContext,
    _path: path::PathBuf,
    res: io::Result<JoshutoDirList>,
) {
    if let Ok(dirlist) = res {
        let history = context.tab_context_mut().curr_tab_mut().history_mut();

        let dir_path = dirlist.file_path().to_path_buf();
        history.insert(dir_path, dirlist);
    }
}
