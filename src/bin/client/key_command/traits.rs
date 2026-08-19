use dizi::error::AppResult;

use crate::config::AppKeyMapping;
use crate::context::AppState;
use crate::ui::AppBackend;

pub trait AppExecute {
    fn execute(
        &self,
        context: &mut AppState,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> AppResult;
}

pub trait AppCommand: AppExecute + std::fmt::Display + std::fmt::Debug {
    fn command(&self) -> &'static str;
}
