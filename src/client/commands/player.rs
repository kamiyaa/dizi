use std::io::Write;
use std::path;

use dizi_lib::error::DiziResult;
use dizi_lib::request::player::*;

use crate::context::AppContext;

macro_rules! simple_server_request {
    ($function_name: ident, $struct_name: ident) => {
        pub fn $function_name(context: &mut AppContext) -> DiziResult<()> {
            let request = $struct_name::new();

            let json = serde_json::to_string(&request).unwrap();

            context.stream.write(json.as_bytes())?;
            context.flush_stream()?;
            Ok(())
        }
    };
}

simple_server_request!(player_get, PlayerGet);
simple_server_request!(player_toggle_play, PlayerTogglePlay);
simple_server_request!(player_toggle_shuffle, PlayerToggleShuffle);
simple_server_request!(player_toggle_repeat, PlayerToggleRepeat);
simple_server_request!(player_toggle_next, PlayerToggleNext);
simple_server_request!(player_play_next, PlayerPlayNext);
simple_server_request!(player_play_previous, PlayerPlayPrevious);

pub fn player_play(context: &mut AppContext, path: path::PathBuf) -> DiziResult<()> {
    let request = PlayerFilePlay::new(path);

    let json = serde_json::to_string(&request).unwrap();

    context.stream.write(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}

pub fn player_volume_increase(context: &mut AppContext, amount: usize) -> DiziResult<()> {
    let request = PlayerVolumeUp::new(amount);

    let json = serde_json::to_string(&request).unwrap();

    context.stream.write(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}

pub fn player_volume_decrease(context: &mut AppContext, amount: usize) -> DiziResult<()> {
    let request = PlayerVolumeDown::new(amount);

    let json = serde_json::to_string(&request).unwrap();

    context.stream.write(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}
