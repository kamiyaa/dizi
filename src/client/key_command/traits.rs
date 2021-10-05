use dizi_commands::error::DiziResult;

use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::ui::TuiBackend;

pub trait AppExecute {
    fn execute(
        &self,
        context: &mut AppContext,
        backend: &mut TuiBackend,
        keymap_t: &AppKeyMapping,
    ) -> DiziResult<()>;
}

pub trait AppCommand: AppExecute + std::fmt::Display + std::fmt::Debug {
    fn command(&self) -> &'static str;
}

pub trait CommandComment {
    fn comment(&self) -> &'static str;
}
