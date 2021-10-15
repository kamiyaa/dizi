use std::path::Path;

use dizi_lib::error::DiziResult;

use crate::events::{ClientRequest, ClientRequestSender};

macro_rules! simple_server_request {
    ($function_name: ident, $enum_name: expr) => {
        pub fn $function_name(client_request_tx: &ClientRequestSender) -> DiziResult<()> {
            client_request_tx.send($enum_name)?;
            Ok(())
        }
    };
}

simple_server_request!(player_pause, ClientRequest::PlayerPause);
simple_server_request!(player_toggle_play, ClientRequest::PlayerTogglePlay);
simple_server_request!(player_get_volume, ClientRequest::PlayerGetVolume);
simple_server_request!(player_play_next, ClientRequest::PlayerPlayNext);
simple_server_request!(player_play_previous, ClientRequest::PlayerPlayPrevious);
simple_server_request!(player_toggle_shuffle, ClientRequest::PlayerToggleShuffle);
simple_server_request!(player_toggle_repeat, ClientRequest::PlayerToggleRepeat);
simple_server_request!(player_toggle_next, ClientRequest::PlayerToggleNext);

pub fn player_play(client_request_tx: &ClientRequestSender, path: &Path) -> DiziResult<()> {
    client_request_tx.send(ClientRequest::PlayerFilePlay(path.to_path_buf()))?;
    Ok(())
}

pub fn player_volume_increase(
    client_request_tx: &ClientRequestSender,
    amount: usize,
) -> DiziResult<()> {
    client_request_tx.send(ClientRequest::PlayerVolumeUp(amount))?;
    Ok(())
}

pub fn player_volume_decrease(
    client_request_tx: &ClientRequestSender,
    amount: usize,
) -> DiziResult<()> {
    client_request_tx.send(ClientRequest::PlayerVolumeDown(amount))?;
    Ok(())
}
