use std::io::{Read, Write};
use std::path;

use dizi_commands::error::DiziResult;
use dizi_commands::structs::{PlayerPause, PlayerPlay, PlayerTogglePlay};

use crate::commands::reload;
use crate::context::AppContext;
use crate::ui::TuiBackend;

pub fn player_play(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    path: path::PathBuf,
) -> DiziResult<()> {
    let request = PlayerPlay::new(path);

    let json = serde_json::to_string(&request).unwrap();

    context.stream.write(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}

pub fn player_toggle_play(context: &mut AppContext, backend: &mut TuiBackend) -> DiziResult<()> {
    let request = PlayerTogglePlay::new();

    let json = serde_json::to_string(&request).unwrap();

    context.stream.write(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}
