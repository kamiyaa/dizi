use signal_hook::consts::signal;
use termion::event::{Event, Key, MouseButton, MouseEvent};

use dizi_lib::error::DiziResult;
use dizi_lib::player::*;
use dizi_lib::response::constants::*;
use dizi_lib::response::player::*;

use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::fs::DirList;
use crate::key_command::{AppExecute, Command, CommandKeybind};
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
    let json_res: serde_json::Map<String, serde_json::Value> = serde_json::from_str(s)?;

    if let Some(serde_json::Value::String(command)) = json_res.get("command") {
        match command.as_str() {
            RESP_SERVER_QUIT => {
                context.quit = QuitType::Server;
            }
            RESP_PLAYLIST_GET => {}
            RESP_PLAYLIST_ADD => {}
            RESP_PLAYLIST_REMOVE => {}
            RESP_PLAYER_GET => {}
            RESP_PLAYER_PLAY => {
                let player_play: PlayerPlay = serde_json::from_str(s)?;
                context
                    .server_state_mut()
                    .player_mut()
                    .set_song(Some(player_play.song));
                context
                    .server_state_mut()
                    .player_mut()
                    .set_player_status(PlayerStatus::Playing);
            }
            RESP_PLAYER_PAUSE => {
                context
                    .server_state_mut()
                    .player_mut()
                    .set_player_status(PlayerStatus::Paused);
            }
            RESP_PLAYER_RESUME => {
                context
                    .server_state_mut()
                    .player_mut()
                    .set_player_status(PlayerStatus::Playing);
            }
            RESP_PLAYER_SHUFFLE_ON => {
                context.server_state_mut().player_mut().set_shuffle(true);
            }
            RESP_PLAYER_SHUFFLE_OFF => {
                context.server_state_mut().player_mut().set_shuffle(false);
            }
            RESP_PLAYER_REPEAT_ON => {
                context.server_state_mut().player_mut().set_repeat(true);
            }
            RESP_PLAYER_REPEAT_OFF => {
                context.server_state_mut().player_mut().set_repeat(false);
            }
            RESP_PLAYER_NEXT_ON => {
                context.server_state_mut().player_mut().set_next(true);
            }
            RESP_PLAYER_NEXT_OFF => {
                context.server_state_mut().player_mut().set_next(false);
            }
            RESP_PLAYER_VOLUME_UPDATE => {
                let player_volume_update: PlayerVolumeUpdate = serde_json::from_str(s)?;
                context
                    .server_state_mut()
                    .player_mut()
                    .set_volume(player_volume_update.volume);
            }
            RESP_PLAYER_PROGRESS_UPDATE => {
                let player_progress_update: PlayerProgressUpdate = serde_json::from_str(s)?;
                context
                    .server_state_mut()
                    .player_mut()
                    .set_duration_played(player_progress_update.duration);
            }
            s => {
                context
                    .message_queue_mut()
                    .push_error(format!("Unknown command: {}", s.to_string()));
            }
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
