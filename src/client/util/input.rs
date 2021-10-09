use std::io;
use std::path;

use signal_hook::consts::signal;
use termion::event::{Event, Key, MouseButton, MouseEvent};
use tui::layout::{Constraint, Direction, Layout};

use crate::commands::cursor_move;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::event::AppEvent;
use crate::fs::DirList;
use crate::history::DirectoryHistory;
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
