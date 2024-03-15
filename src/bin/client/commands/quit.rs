use dizi::error::DiziResult;
use dizi::request::client::ClientRequest;

use crate::context::{AppContext, QuitType};
use crate::util::request::send_client_request;

pub fn close(context: &mut AppContext) -> DiziResult {
    context.quit = QuitType::Normal;
    Ok(())
}

pub fn server_quit(context: &mut AppContext) -> DiziResult {
    let request = ClientRequest::ServerQuit;
    let _ = send_client_request(context, &request);
    context.quit = QuitType::Server;
    Ok(())
}
