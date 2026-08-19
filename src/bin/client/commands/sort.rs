use dizi::error::AppResult;

use crate::config::option::SortType;
use crate::context::AppState;
use crate::history::DirectoryHistory;

use super::reload;

pub fn set_sort(context: &mut AppState, method: SortType) -> AppResult {
    context
        .config_mut()
        .sort_options_mut()
        .set_sort_method(method);
    for tab in context.tab_state_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
    }
    refresh(context)
}

pub fn toggle_reverse(context: &mut AppState) -> AppResult {
    let reversed = !context.config_ref().sort_options_ref().reverse;
    context.config_mut().sort_options_mut().reverse = reversed;

    for tab in context.tab_state_mut().iter_mut() {
        tab.history_mut().depreciate_all_entries();
    }
    refresh(context)
}

fn refresh(context: &mut AppState) -> AppResult {
    reload::soft_reload(context.tab_state_ref().index, context)?;
    Ok(())
}
