use dizi_commands::error::DiziResult;

use crate::context::AppContext;
use crate::history::DirectoryHistory;

use super::reload;

pub fn _toggle_hidden(context: &mut AppContext) {
    let opposite = !context.config_ref().display_options_ref().show_hidden();
    context
        .config_mut()
        .display_options_mut()
        .set_show_hidden(opposite);

    context.history_mut().depreciate_all_entries();
}

pub fn toggle_hidden(context: &mut AppContext) -> DiziResult<()> {
    _toggle_hidden(context);
    reload::reload_dirlist(context)
}
