use std::collections::HashMap;
use std::path::PathBuf;

use dizi_commands::api_command::ApiCommand;
use dizi_commands::error::DiziResult;

use crate::commands::*;
use crate::context::AppContext;

pub fn run_command(context: &mut AppContext, command: ApiCommand, json_map: &HashMap<String, String>) -> DiziResult<()> {
    match command {
        ApiCommand::Quit => {},
        ApiCommand::PlayerPlay => match json_map.get("path").map(|k| PathBuf::from(k)) {
            Some(p) => {
                player_play(context, &p)?;
            }
            None => {}
        },
        _ => {},
    }
    Ok(())
}
