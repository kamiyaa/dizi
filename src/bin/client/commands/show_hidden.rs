use dizi::error::AppResult;

use crate::context::AppState;
use crate::fs::DirectoryHistory;

use super::reload;

pub fn _toggle_hidden(context: &mut AppState) {
    let display_options = &mut context.config.client.display_options;
    display_options.show_hidden = !display_options.show_hidden;

    for tab in context.tab_state.iter_mut() {
        tab.history.depreciate_all_entries();
        if let Some(s) = tab.curr_list_mut() {
            s.depreciate();
        }
    }
}

pub fn toggle_hidden(context: &mut AppState) -> AppResult {
    _toggle_hidden(context);
    reload::reload_dirlist(context)
}
