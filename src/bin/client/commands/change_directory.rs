use std::io;
use std::path;

use dizi::error::AppResult;

use crate::commands::reload;
use crate::config::option::WidgetType;
use crate::context::AppState;
use crate::fs::DirectoryHistory;

pub fn cd(path: &path::Path, context: &mut AppState) -> io::Result<()> {
    std::env::set_current_dir(path)?;
    context.tab_state.curr_tab_mut().set_cwd(path);
    Ok(())
}

pub fn change_directory(context: &mut AppState, path: &path::Path) -> AppResult {
    let new_cwd = if path.is_absolute() {
        path.canonicalize()?
    } else {
        let mut new_cwd = std::env::current_dir()?;
        new_cwd.push(path.canonicalize()?);
        new_cwd
    };

    cd(new_cwd.as_path(), context)?;
    let options = context.config.client.display_options.clone();
    let ui_state = context.ui_state.clone();
    context.tab_state.curr_tab_mut().history.populate_to_root(
        new_cwd.as_path(),
        &ui_state,
        &options,
    )?;
    Ok(())
}

// ParentDirectory command
pub fn parent_directory(context: &mut AppState) -> AppResult {
    if context.get_view_widget() != WidgetType::FileBrowser {
        return Ok(());
    }

    if let Some(parent) = context
        .tab_state
        .curr_tab_ref()
        .get_cwd()
        .parent()
        .map(|p| p.to_path_buf())
    {
        std::env::set_current_dir(&parent)?;
        context.tab_state.curr_tab_mut().set_cwd(parent.as_path());
        reload::soft_reload(context.tab_state.index, context)?;
    }
    Ok(())
}
