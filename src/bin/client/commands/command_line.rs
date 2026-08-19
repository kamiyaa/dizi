use std::str::FromStr;

use dizi::error::AppResult;

use crate::config::AppKeyMapping;
use crate::context::AppState;
use crate::key_command::{AppExecute, Command};
use crate::ui::AppBackend;
use crate::ui::views::{DummyListener, TuiTextField};

pub fn read_and_execute(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    keymap_t: &AppKeyMapping,
    prefix: &str,
    suffix: &str,
) -> AppResult {
    app_state.flush_event();

    let mut listener = DummyListener {};
    let user_input: Option<String> = TuiTextField::default()
        .prompt(":")
        .prefix(prefix)
        .suffix(suffix)
        .get_input(app_state, backend, &mut listener);

    if let Some(s) = user_input {
        let trimmed = s.trim_start();
        let _ = app_state.commandline_state_mut().history_mut().add(trimmed);

        let command = Command::from_str(trimmed)?;
        command.execute(app_state, backend, keymap_t)
    } else {
        Ok(())
    }
}
