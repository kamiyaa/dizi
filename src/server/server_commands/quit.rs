use dizi_lib::error::DiziResult;

use crate::context::{AppContext, QuitType};

pub fn quit_server(context: &mut AppContext) -> DiziResult<()> {
    context.quit = QuitType::Server;
    Ok(())
}
