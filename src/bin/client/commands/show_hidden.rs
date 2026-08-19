use dizi::error::AppResult;

use crate::context::AppState;
use crate::history::DirectoryHistory;

use super::reload;

pub fn _toggle_hidden(context: &mut AppState) {
    let opposite = !context.config_ref().display_options_ref().show_hidden();
    context
        .config_mut()
        .display_options_mut()
        .set_show_hidden(opposite);

    for tab in context.tab_state_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
        if let Some(s) = tab.curr_list_mut() {
            s.depreciate();
        }
    }
}

pub fn toggle_hidden(context: &mut AppState) -> AppResult {
    _toggle_hidden(context);
    reload::reload_dirlist(context)
}
