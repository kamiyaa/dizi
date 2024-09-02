use std::path::Path;

use dizi::error::DiziResult;
use dizi::player::PlayerStatus;

use crate::context::AppContext;
use crate::server_util::run_on_song_change;
use crate::traits::AudioPlayer;

pub fn player_play(context: &mut AppContext, path: &Path) -> DiziResult {
    context.player.play_directory(path)?;

    run_on_song_change(context);
    Ok(())
}

pub fn player_pause(context: &mut AppContext) -> DiziResult {
    context.player.pause()
}

pub fn player_resume(context: &mut AppContext) -> DiziResult {
    context.player.resume()
}

pub fn player_toggle_play(context: &mut AppContext) -> DiziResult<PlayerStatus> {
    let status = context.player.toggle_play()?;
    Ok(status)
}

pub fn player_get_volume(context: &mut AppContext) -> usize {
    context.player.get_volume()
}

pub fn player_set_volume(context: &mut AppContext, volume: usize) -> DiziResult {
    context.player.set_volume(volume)?;
    Ok(())
}

pub fn player_volume_increase(context: &mut AppContext, amount: usize) -> DiziResult<usize> {
    let volume = player_get_volume(context);

    let volume = if volume + amount > 100 {
        100
    } else {
        volume + amount
    };
    player_set_volume(context, volume)?;

    tracing::debug!("volume is now: {volume}");
    Ok(volume)
}

pub fn player_volume_decrease(context: &mut AppContext, amount: usize) -> DiziResult<usize> {
    let volume = player_get_volume(context);

    let volume = if amount > volume { 0 } else { volume - amount };
    player_set_volume(context, volume)?;

    tracing::debug!("volume is now: {volume}");
    Ok(volume)
}

pub fn player_play_again(context: &mut AppContext) -> DiziResult {
    context.player.play_again()?;
    run_on_song_change(context);
    Ok(())
}

pub fn player_play_next(context: &mut AppContext) -> DiziResult {
    context.player.play_next()?;
    run_on_song_change(context);
    Ok(())
}

pub fn player_play_previous(context: &mut AppContext) -> DiziResult {
    context.player.play_previous()?;
    run_on_song_change(context);
    Ok(())
}
