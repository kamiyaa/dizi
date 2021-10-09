use std::collections::HashMap;
use std::path::PathBuf;

use dizi_commands::api_command::ApiCommand;
use dizi_commands::error::DiziResult;

use crate::commands::*;
use crate::context::{AppContext, PlayerContext};

pub fn run_command(
    context: &mut AppContext,
    command: ApiCommand,
    json_map: &HashMap<String, String>,
) -> DiziResult<()> {
    eprintln!("Command received: {:?}", command);
    match command {
        ApiCommand::Quit => {}
        ApiCommand::PlayerPlay => match json_map.get("path").map(|k| PathBuf::from(k)) {
            Some(p) => {
                player_play(context, &p)?;
            }
            None => {}
        },
        ApiCommand::PlayerPause => {
            player_pause(context)?;
        }
        ApiCommand::PlayerTogglePlay => {
            player_toggle_play(context)?;
        }
        _ => {}
    }
    Ok(())
}
