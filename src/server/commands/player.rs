use std::path::Path;

use dizi_commands::error::DiziResult;

use crate::context::AppContext;

pub fn player_play(context: &mut AppContext, path: &Path) -> DiziResult<()> {
    let res = context.player_context_mut().player_mut().play(path);
    eprintln!("Playing {:?} res: {:?}", path, res);
    match res {
        Ok(handle) => handle.join(),
        Err(e) => return Err(e),
    };
    Ok(())
}
