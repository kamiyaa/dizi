use std::io::Write;

use dizi::error::AppResult;
use dizi::request::client::ClientRequest;

use crate::context::AppState;

pub fn send_client_request(context: &mut AppState, request: &ClientRequest) -> AppResult {
    let json = serde_json::to_string(&request)?;

    context.stream.write_all(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}
