use dizi::error::AppResult;

use crate::config::option::SortMethod;
use crate::context::AppState;
use crate::fs::DirectoryHistory;

use super::reload;

pub fn set_sort(context: &mut AppState, method: SortMethod) -> AppResult {
    context
        .config
        .client
        .display_options
        .sort_options
        .set_sort_method(method);
    for tab in context.tab_state.iter_mut() {
        tab.history.depreciate_all_entries();
    }
    refresh(context)
}

pub fn toggle_reverse(context: &mut AppState) -> AppResult {
    let reversed = !context.config.client.display_options.sort_options.reverse;
    context.config.client.display_options.sort_options.reverse = reversed;

    for tab in context.tab_state.iter_mut() {
        tab.history.depreciate_all_entries();
    }
    refresh(context)
}

fn refresh(context: &mut AppState) -> AppResult {
    reload::soft_reload(context.tab_state.index, context)?;
    Ok(())
}
