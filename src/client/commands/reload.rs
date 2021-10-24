use dizi_lib::error::DiziResult;

use crate::context::AppContext;
use crate::history::create_dirlist_with_history;

// reload only if we have a queued reload
pub fn soft_reload(context: &mut AppContext) -> std::io::Result<()> {
    let mut paths = Vec::with_capacity(1);
    if let Some(curr_list) = context.curr_list_ref() {
        if curr_list.need_update() {
            paths.push(curr_list.file_path().to_path_buf());
        }
    }

    if !paths.is_empty() {
        let options = context.config_ref().display_options_ref().clone();
        let history = context.history_mut();
        for path in paths {
            let new_dirlist = create_dirlist_with_history(history, path.as_path(), &options)?;
            history.insert(path, new_dirlist);
        }
    }
    Ok(())
}

pub fn reload(context: &mut AppContext) -> std::io::Result<()> {
    let mut paths = Vec::with_capacity(1);
    if let Some(curr_list) = context.curr_list_ref() {
        paths.push(curr_list.file_path().to_path_buf());
    }

    if !paths.is_empty() {
        let options = context.config_ref().display_options_ref().clone();
        let history = context.history_mut();
        for path in paths {
            let new_dirlist = create_dirlist_with_history(history, path.as_path(), &options)?;
            history.insert(path, new_dirlist);
        }
    }
    context
        .message_queue_mut()
        .push_success("Directory listing reloaded!".to_string());
    Ok(())
}

pub fn reload_dirlist(context: &mut AppContext) -> DiziResult<()> {
    reload(context)?;
    Ok(())
}
