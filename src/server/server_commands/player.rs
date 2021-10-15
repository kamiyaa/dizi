use std::path::Path;

use dizi_lib::error::DiziResult;
use dizi_lib::player::PlayerStatus;

use crate::context::AppContext;

pub fn player_play(context: &mut AppContext, path: &Path) -> DiziResult<()> {
    context.player_context_mut().player_mut().play(path)
}

pub fn player_pause(context: &mut AppContext) -> DiziResult<()> {
    context.player_context_mut().player_mut().pause()
}

pub fn player_resume(context: &mut AppContext) -> DiziResult<()> {
    context.player_context_mut().player_mut().resume()
}

pub fn player_toggle_play(context: &mut AppContext) -> DiziResult<PlayerStatus> {
    let status = context.player_context_mut().player_mut().toggle_play()?;
    Ok(status)
}

pub fn player_get_volume(context: &mut AppContext) -> f32 {
    context.player_context_mut().player_mut().get_volume()
}

pub fn player_set_volume(context: &mut AppContext, volume: f32) -> DiziResult<()> {
    context
        .player_context_mut()
        .player_mut()
        .set_volume(volume)?;
    Ok(())
}

pub fn player_volume_increase(context: &mut AppContext, amount: usize) -> DiziResult<usize> {
    let volume = player_get_volume(context);

    let amount: f32 = amount as f32 / 100.0;
    let volume = if volume + amount > 1.0 {
        1.0
    } else {
        volume + amount
    };
    player_set_volume(context, volume)?;

    let volume: usize = (volume * 100.0) as usize;
    eprintln!("volume is now: {}", volume);
    Ok(volume)
}

pub fn player_volume_decrease(context: &mut AppContext, amount: usize) -> DiziResult<usize> {
    let volume = player_get_volume(context);

    let amount: f32 = amount as f32 / 100.0;
    let volume = if volume - amount < 0.0 {
        0.0
    } else {
        volume - amount
    };
    player_set_volume(context, volume)?;

    let volume: usize = (volume * 100.0) as usize;
    eprintln!("volume is now: {}", volume);
    Ok(volume)
}

pub fn player_play_next(context: &mut AppContext) -> DiziResult<()> {
    Ok(())
}

pub fn player_play_previous(context: &mut AppContext) -> DiziResult<()> {
    Ok(())
}
