use std::io::Write;
use std::path;

use dizi_lib::error::DiziResult;
use dizi_lib::player::*;

use crate::context::AppContext;

pub fn player_get(context: &mut AppContext, path: path::PathBuf) -> DiziResult<()> {
    let request = PlayerGet::new();

    let json = serde_json::to_string(&request).unwrap();

    context.stream.write(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}

pub fn player_play(context: &mut AppContext, path: path::PathBuf) -> DiziResult<()> {
    let request = PlayerPlay::new(path);

    let json = serde_json::to_string(&request).unwrap();

    context.stream.write(json.as_bytes())?;
    context.flush_stream()?;
    Ok(())
}

pub fn player_toggle_play(context: &mut AppContext) -> DiziResult<()> {
    let request = PlayerTogglePlay::new();

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
