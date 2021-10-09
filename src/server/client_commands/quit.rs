use std::path::Path;

use dizi_lib::error::DiziResult;

use crate::events::{ClientEvent, ClientEventSender};

pub fn quit_server(server_req: &ClientEventSender) -> DiziResult<()> {
    server_req.send(ClientEvent::Quit)?;
    Ok(())
}
