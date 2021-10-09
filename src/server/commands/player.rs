use std::path::Path;

use dizi_lib::error::DiziResult;

use crate::context::AppContext;

pub fn player_play(context: &mut AppContext, path: &Path) -> DiziResult<()> {
    context.player_context_mut().player_mut().play(path)
}

pub fn player_pause(context: &mut AppContext) -> DiziResult<()> {
    context.player_context_mut().player_mut().pause()
}

pub fn player_toggle_play(context: &mut AppContext) -> DiziResult<()> {
    context.player_context_mut().player_mut().toggle_play()
}

pub fn player_get_volume(context: &mut AppContext, amount: usize) -> DiziResult<f32> {
    let volume = context.player_context_mut().player_mut().get_volume()?;
    Ok(volume)
}

pub fn player_volume_increase(context: &mut AppContext, amount: usize) -> DiziResult<()> {
    let volume = context.player_context_mut().player_mut().get_volume()?;

    let amount: f32 = amount as f32 / 100.0;

    let volume = if volume + amount > 1.0 {
        1.0
    } else {
        volume + amount
    };
    context
        .player_context_mut()
        .player_mut()
        .set_volume(volume)?;
    eprintln!("volume is now: {}", volume);
    Ok(())
}

pub fn player_volume_decrease(context: &mut AppContext, amount: usize) -> DiziResult<()> {
    let volume = context.player_context_mut().player_mut().get_volume()?;

    let amount: f32 = amount as f32 / 100.0;

    let volume = if volume - amount < 0.0 {
        0.0
    } else {
        volume - amount
    };
    context
        .player_context_mut()
        .player_mut()
        .set_volume(volume)?;
    eprintln!("volume is now: {}", volume);
    Ok(())
}
