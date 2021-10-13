use std::path::Path;

use dizi_lib::error::DiziResult;
use dizi_lib::song::Song;

use crate::events::{ClientRequest, ClientRequestSender};

pub fn player_play(server_req: &ClientRequestSender, path: &Path) -> DiziResult<()> {
    server_req.send(ClientRequest::PlayerPlay(path.to_path_buf()))?;
    Ok(())
}

pub fn player_pause(server_req: &ClientRequestSender) -> DiziResult<()> {
    server_req.send(ClientRequest::PlayerPause)?;
    Ok(())
}

pub fn player_toggle_play(server_req: &ClientRequestSender) -> DiziResult<()> {
    server_req.send(ClientRequest::PlayerTogglePlay)?;
    Ok(())
}

pub fn player_get_volume(server_req: &ClientRequestSender, amount: usize) -> DiziResult<()> {
    server_req.send(ClientRequest::PlayerGetVolume)?;
    Ok(())
}

pub fn player_volume_increase(server_req: &ClientRequestSender, amount: usize) -> DiziResult<()> {
    server_req.send(ClientRequest::PlayerVolumeUp(amount))?;
    Ok(())
}

pub fn player_volume_decrease(server_req: &ClientRequestSender, amount: usize) -> DiziResult<()> {
    server_req.send(ClientRequest::PlayerVolumeDown(amount))?;
    Ok(())
}
