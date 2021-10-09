use crate::commands::reload;
use crate::context::AppContext;
use dizi_commands::error::DiziResult;

pub fn parent_directory_helper(context: &mut AppContext) -> std::io::Result<()> {
    if let Some(parent) = context.cwd().parent().map(|p| p.to_path_buf()) {
        std::env::set_current_dir(&parent)?;
        context.set_cwd(parent.as_path());
    }
    Ok(())
}

pub fn parent_directory(context: &mut AppContext) -> DiziResult<()> {
    parent_directory_helper(context)?;
    reload::soft_reload(context)?;
    Ok(())
}
