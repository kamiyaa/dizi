use std::io;
use std::path;

use dizi::error::DiziResult;

use crate::commands::reload;
use crate::config::option::WidgetType;
use crate::context::AppContext;
use crate::history::DirectoryHistory;

pub fn cd(path: &path::Path, context: &mut AppContext) -> io::Result<()> {
    std::env::set_current_dir(path)?;
    context.tab_context_mut().curr_tab_mut().set_cwd(path);
    Ok(())
}

pub fn change_directory(context: &mut AppContext, path: &path::Path) -> DiziResult {
    let new_cwd = if path.is_absolute() {
        path.canonicalize()?
    } else {
        let mut new_cwd = std::env::current_dir()?;
        new_cwd.push(path.canonicalize()?);
        new_cwd
    };

    cd(new_cwd.as_path(), context)?;
    let options = context.config_ref().display_options_ref().clone();
    let ui_context = context.ui_context_ref().clone();
    context
        .tab_context_mut()
        .curr_tab_mut()
        .history_mut()
        .populate_to_root(new_cwd.as_path(), &ui_context, &options)?;
    Ok(())
}

// ParentDirectory command
pub fn parent_directory(context: &mut AppContext) -> DiziResult {
    if context.get_view_widget() != WidgetType::FileBrowser {
        return Ok(());
    }

    if let Some(parent) = context
        .tab_context_ref()
        .curr_tab_ref()
        .cwd()
        .parent()
        .map(|p| p.to_path_buf())
    {
        std::env::set_current_dir(&parent)?;
        context
            .tab_context_mut()
            .curr_tab_mut()
            .set_cwd(parent.as_path());
        reload::soft_reload(context.tab_context_ref().index, context)?;
    }
    Ok(())
}
