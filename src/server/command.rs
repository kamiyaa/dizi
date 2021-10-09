use std::collections::HashMap;
use std::path::PathBuf;

use dizi_lib::constants::*;
use dizi_lib::error::DiziResult;
use dizi_lib::player;

use crate::commands::*;
use crate::context::{AppContext, PlayerContext};

pub fn run_command(context: &mut AppContext, s: &str) -> DiziResult<()> {
    let json_res: Result<serde_json::Map<String, serde_json::Value>, serde_json::Error> =
        serde_json::from_str(s);

    eprintln!("s: '{}'", s);

    match json_res {
        Ok(json_map) => match json_map.get("command") {
            Some(serde_json::Value::String(command)) => match command.as_str() {
                API_SERVER_QUIT => {
                    quit_server(context);
                }
                API_PLAYER_PLAY => {
                    let cmd: player::PlayerPlay = serde_json::from_str(s).unwrap();
                    player_play(context, &cmd.path)?;
                }
                API_PLAYER_PAUSE => {
                    player_pause(context)?;
                }
                API_PLAYER_TOGGLE_PLAY => {
                    player_toggle_play(context)?;
                }
                API_PLAYER_VOLUME_UP => {
                    let cmd: player::PlayerVolumeUp = serde_json::from_str(s).unwrap();
                    player_volume_increase(context, cmd.amount)?;
                }
                API_PLAYER_VOLUME_DOWN => {
                    let cmd: player::PlayerVolumeDown = serde_json::from_str(s).unwrap();
                    player_volume_decrease(context, cmd.amount)?;
                }
                s => {
                    eprintln!("Error: '{:?}' not implemented", s);
                }
            },
            _ => {}
        },
        _ => {}
    }
    Ok(())
}
