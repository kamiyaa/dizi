use dizi_lib::error::DiziResult;

use crate::events::{ClientRequest, ClientRequestSender};

pub fn quit_server(server_req: &ClientRequestSender) -> DiziResult<()> {
    server_req.send(ClientRequest::Quit)?;
    Ok(())
}
