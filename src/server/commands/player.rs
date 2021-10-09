use std::path::Path;

use dizi_commands::error::DiziResult;

use crate::context::AppContext;

pub fn player_play(context: &mut AppContext, path: &Path) -> DiziResult<()> {
    let res = context.player_context_mut().player_mut().play(path);
    eprintln!("Playing {:?} res: {:?}", path, res);
    Ok(())
}

pub fn player_pause(context: &mut AppContext) -> DiziResult<()> {
    let res = context.player_context_mut().player_mut().pause();
    Ok(())
}

pub fn player_toggle_play(context: &mut AppContext) -> DiziResult<()> {
    let res = context.player_context_mut().player_mut().toggle_play();
    Ok(())
}
