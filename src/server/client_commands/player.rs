use std::path::Path;

use dizi_lib::error::DiziResult;
use dizi_lib::song::Song;

use crate::events::{ClientEvent, ClientEventSender};

pub fn player_play(server_req: &ClientEventSender, path: &Path) -> DiziResult<()> {
    let song = Song::new(path)?;
    server_req.send(ClientEvent::PlayerPlay(song))?;
    Ok(())
}

pub fn player_pause(server_req: &ClientEventSender) -> DiziResult<()> {
    server_req.send(ClientEvent::PlayerPause)?;
    Ok(())
}

pub fn player_toggle_play(server_req: &ClientEventSender) -> DiziResult<()> {
    server_req.send(ClientEvent::PlayerTogglePlay)?;
    Ok(())
}

pub fn player_get_volume(server_req: &ClientEventSender, amount: usize) -> DiziResult<()> {
    server_req.send(ClientEvent::PlayerGetVolume)?;
    Ok(())
}

pub fn player_volume_increase(server_req: &ClientEventSender, amount: usize) -> DiziResult<()> {
    server_req.send(ClientEvent::PlayerVolumeUp(amount))?;
    Ok(())
}

pub fn player_volume_decrease(server_req: &ClientEventSender, amount: usize) -> DiziResult<()> {
    server_req.send(ClientEvent::PlayerVolumeDown(amount))?;
    Ok(())
}
