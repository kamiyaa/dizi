use dizi::error::AppResult;

use crate::context::AppState;
use crate::fs::create_dirlist_with_history;

// reload only if we have a queued reload
pub fn soft_reload(index: usize, context: &mut AppState) -> std::io::Result<()> {
    let mut paths = Vec::with_capacity(3);
    if let Some(curr_tab) = context.tab_state.tab_ref(index) {
        if let Some(curr_list) = curr_tab.curr_list_ref()
            && curr_list.need_update()
        {
            paths.push(curr_list.file_path().to_path_buf());
        }
        if let Some(curr_list) = curr_tab.parent_list_ref()
            && curr_list.need_update()
        {
            paths.push(curr_list.file_path().to_path_buf());
        }
        if let Some(curr_list) = curr_tab.child_list_ref()
            && curr_list.need_update()
        {
            paths.push(curr_list.file_path().to_path_buf());
        }
    }

    if !paths.is_empty() {
        let options = context.config.client.display_options.clone();
        if let Some(history) = context.tab_state.tab_mut(index).map(|t| &mut t.history) {
            for path in paths {
                let new_dirlist = create_dirlist_with_history(history, path.as_path(), &options)?;
                history.insert(path, new_dirlist);
            }
        }
    }
    Ok(())
}

pub fn reload(context: &mut AppState, index: usize) -> std::io::Result<()> {
    let mut paths = Vec::with_capacity(3);
    if let Some(curr_tab) = context.tab_state.tab_ref(index) {
        if let Some(curr_list) = curr_tab.curr_list_ref() {
            paths.push(curr_list.file_path().to_path_buf());
        }
        if let Some(curr_list) = curr_tab.parent_list_ref() {
            paths.push(curr_list.file_path().to_path_buf());
        }
        if let Some(curr_list) = curr_tab.child_list_ref() {
            paths.push(curr_list.file_path().to_path_buf());
        }
    }

    if !paths.is_empty() {
        let options = context.config.client.display_options.clone();
        if let Some(history) = context.tab_state.tab_mut(index).map(|t| &mut t.history) {
            for path in paths {
                let new_dirlist = create_dirlist_with_history(history, path.as_path(), &options)?;
                history.insert(path, new_dirlist);
            }
        }
    }
    context
        .message_queue
        .push_success("Directory listing reloaded!".to_string());
    Ok(())
}

pub fn reload_dirlist(context: &mut AppState) -> AppResult {
    reload(context, context.tab_state.index)?;
    Ok(())
}
