use dizi::error::DiziResult;
use dizi::request::client::ClientRequest;

use crate::context::AppContext;
use crate::utils::request::send_client_request;
use crate::CommandArgs;

pub fn run_control(context: &mut AppContext, args: &CommandArgs) -> DiziResult {
    let request = if args.exit {
        Some(ClientRequest::ServerQuit)
    } else if args.next {
        Some(ClientRequest::PlayerPlayNext)
    } else if args.previous {
        Some(ClientRequest::PlayerPlayPrevious)
    } else if args.pause {
        Some(ClientRequest::PlayerPause)
    } else if args.resume {
        Some(ClientRequest::PlayerResume)
    } else if args.toggle_play {
        Some(ClientRequest::PlayerTogglePlay)
    } else {
        None
    };
    if let Some(request) = request {
        send_client_request(context, &request)?;
    }
    Ok(())
}
