use std::io::Write;

use dizi::error::DiziResult;
use dizi::request::client::ClientRequest;

use crate::context::AppContext;

pub fn send_client_request(context: &mut AppContext, request: &ClientRequest) -> DiziResult {
    let json = serde_json::to_string(&request)?;

    context.stream.write_all(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}
