
use crate::context::{AppContext, QuitType};
use dizi_commands::error::DiziResult;

pub fn close(context: &mut AppContext) -> DiziResult<()> {
    context.quit = QuitType::Normal;
    Ok(())
}

pub fn quit_server(context: &mut AppContext) -> DiziResult<()> {
    context.quit = QuitType::Server;
    Ok(())
}
