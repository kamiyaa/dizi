use dizi::error::AppResult;
use dizi::request::client::ClientRequest;

use crate::context::{AppState, QuitType};
use crate::utils::request::send_client_request;

pub fn close(context: &mut AppState) -> AppResult {
    context.quit = QuitType::Normal;
    Ok(())
}

pub fn server_quit(context: &mut AppState) -> AppResult {
    let request = ClientRequest::ServerQuit;
    let _ = send_client_request(context, &request);
    context.quit = QuitType::Server;
    Ok(())
}
