use std::io::Write;

use dizi_lib::error::DiziResult;
use dizi_lib::request::server::*;

use crate::context::{AppContext, QuitType};

pub fn close(context: &mut AppContext) -> DiziResult<()> {
    context.quit = QuitType::Normal;
    Ok(())
}

pub fn quit_server(context: &mut AppContext) -> DiziResult<()> {
    let request = ServerQuit::new();

    let json = serde_json::to_string(&request).unwrap();

    context.stream.write(json.as_bytes())?;
    context.flush_stream()?;

    context.quit = QuitType::Server;
    Ok(())
}
