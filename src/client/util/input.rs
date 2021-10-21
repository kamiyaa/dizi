use signal_hook::consts::signal;
use termion::event::{Event, Key, MouseButton, MouseEvent};

use dizi_lib::error::DiziResult;
use dizi_lib::player::*;
use dizi_lib::response::server::ServerBroadcastEvent;

use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::fs::DirList;
use crate::key_command::{Command, CommandKeybind};
use crate::ui;
use crate::ui::views::TuiCommandMenu;
use crate::util::format;

pub fn get_input_while_composite<'a>(
    backend: &mut ui::TuiBackend,
    context: &mut AppContext,
    keymap: &'a AppKeyMapping,
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
                        event => match keymap.as_ref().get(&event) {
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

pub fn process_server_event(context: &mut AppContext, s: &str) -> DiziResult<()> {
    let server_broadcast_event: ServerBroadcastEvent = serde_json::from_str(s)?;

    match server_broadcast_event {
        ServerBroadcastEvent::ServerQuit => {
            context.quit = QuitType::Server;
        }
        ServerBroadcastEvent::PlayerState { state } => {
            context.server_state_mut().set_player_state(state);
        }
        ServerBroadcastEvent::PlayerFilePlay { song } => {
            context
                .server_state_mut()
                .player_state_mut()
                .set_song(Some(song));
            context
                .server_state_mut()
                .player_state_mut()
                .set_player_status(PlayerStatus::Playing);
        }
        ServerBroadcastEvent::PlayerPause => {
            context
                .server_state_mut()
                .player_state_mut()
                .set_player_status(PlayerStatus::Paused);
        }
        ServerBroadcastEvent::PlayerResume => {
            context
                .server_state_mut()
                .player_state_mut()
                .set_player_status(PlayerStatus::Playing);
        }
        ServerBroadcastEvent::PlayerShuffle { on } => {
            context
                .server_state_mut()
                .player_state_mut()
                .set_shuffle(on);
        }
        ServerBroadcastEvent::PlayerRepeat { on } => {
            context.server_state_mut().player_state_mut().set_repeat(on);
        }
        ServerBroadcastEvent::PlayerNext { on } => {
            context.server_state_mut().player_state_mut().set_next(on);
        }
        ServerBroadcastEvent::PlayerVolumeUpdate { volume } => {
            context
                .server_state_mut()
                .player_state_mut()
                .set_volume(volume);
        }
        ServerBroadcastEvent::PlayerProgressUpdate { elapsed } => {
            context
                .server_state_mut()
                .player_state_mut()
                .set_elapsed(elapsed);
        }
        ServerBroadcastEvent::PlaylistAppend { song } => {
            context
                .server_state_mut()
                .player_state_mut()
                .playlist_mut()
                .append_song(song);
        }
        ServerBroadcastEvent::PlaylistRemove { index } => {
            context
                .server_state_mut()
                .player_state_mut()
                .playlist_mut()
                .remove_song(index);
        }
        s => {
            context
                .message_queue_mut()
                .push_error(format!("Unknown command: {:?}", s));
        }
    }
    Ok(())
}

pub fn process_noninteractive(event: AppEvent, context: &mut AppContext) {
    match event {
        AppEvent::PreviewDir(Ok(dirlist)) => process_dir_preview(context, dirlist),
        AppEvent::Signal(signal::SIGWINCH) => {}
        _ => {}
    }
}

pub fn process_dir_preview(context: &mut AppContext, dirlist: DirList) {
    let history = context.history_mut();

    let dir_path = dirlist.file_path().to_path_buf();
    history.insert(dir_path, dirlist);
}
