use dizi_lib::error::DiziResult;
use dizi_lib::request::constants::*;
use dizi_lib::request::player;

use crate::client_commands::*;
use crate::events::ClientRequestSender;

pub fn run_command(server_req: &ClientRequestSender, s: &str) -> DiziResult<()> {
    let json_res: Result<serde_json::Map<String, serde_json::Value>, serde_json::Error> =
        serde_json::from_str(s);

    eprintln!("client_request: '{}'", s);

    match json_res {
        Ok(json_map) => match json_map.get("command") {
            Some(serde_json::Value::String(command)) => match command.as_str() {
                API_SERVER_QUIT => {
                    quit_server(server_req);
                }
                API_PLAYER_FILE_PLAY => {
                    let cmd: player::PlayerFilePlay = serde_json::from_str(s)?;
                    player_play(server_req, &cmd.path)?;
                }
                API_PLAYER_PAUSE => {
                    player_pause(server_req)?;
                }
                API_PLAYER_TOGGLE_PLAY => {
                    player_toggle_play(server_req)?;
                }
                API_PLAYER_TOGGLE_SHUFFLE => {
                    player_toggle_shuffle(server_req)?;
                }
                API_PLAYER_TOGGLE_REPEAT => {
                    player_toggle_repeat(server_req)?;
                }
                API_PLAYER_TOGGLE_NEXT => {
                    player_toggle_next(server_req)?;
                }
                API_PLAYER_VOLUME_UP => {
                    let cmd: player::PlayerVolumeUp = serde_json::from_str(s).unwrap();
                    player_volume_increase(server_req, cmd.amount)?;
                }
                API_PLAYER_VOLUME_DOWN => {
                    let cmd: player::PlayerVolumeDown = serde_json::from_str(s).unwrap();
                    player_volume_decrease(server_req, cmd.amount)?;
                }
                API_PLAYER_PLAY_NEXT => {
                    player_play_next(server_req)?;
                }
                API_PLAYER_PLAY_PREVIOUS => {
                    player_play_previous(server_req)?;
                }
                s => {
                    eprintln!("Error: '{:?}' not implemented", s);
                }
            },
            _ => {
                eprintln!("Error: no command");
            }
        },
        _ => {
            eprintln!("Error: invalid JSON");
        }
    }
    Ok(())
}
