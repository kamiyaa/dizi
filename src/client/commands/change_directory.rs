use std::path;
use std::io;

use dizi_commands::error::DiziResult;

use crate::context::AppContext;
use crate::history::DirectoryHistory;

pub fn cd(path: &path::Path, context: &mut AppContext) -> io::Result<()> {
    std::env::set_current_dir(path)?;
    context.set_cwd(path);
    Ok(())
}

fn _change_directory(path: &path::Path, context: &mut AppContext) -> io::Result<()> {
    cd(path, context)?;
    let options = context.config_ref().display_options_ref().clone();
    context
        .history_mut()
        .populate_to_root(path, &options)?;

    Ok(())
}

pub fn change_directory(context: &mut AppContext, path: &path::Path) -> DiziResult<()> {
    let new_cwd = if path.is_absolute() {
        path.canonicalize()?
    } else {
        let mut new_cwd = std::env::current_dir()?;
        new_cwd.push(path.canonicalize()?);
        new_cwd
    };

    _change_directory(new_cwd.as_path(), context)?;
    Ok(())
}
