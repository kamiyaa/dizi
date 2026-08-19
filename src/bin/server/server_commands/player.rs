use std::path::Path;

use dizi::error::AppResult;
use dizi::player::PlayerStatus;

use crate::context::AppContext;
use crate::server_util::run_on_song_change;
use crate::traits::AudioPlayer;

pub fn player_play(context: &mut AppContext, path: &Path) -> AppResult {
    context.player.play_directory(path)?;

    run_on_song_change(context);
    Ok(())
}

pub fn player_pause(context: &mut AppContext) -> AppResult {
    context.player.pause()
}

pub fn player_resume(context: &mut AppContext) -> AppResult {
    context.player.resume()
}

pub fn player_toggle_play(context: &mut AppContext) -> AppResult<PlayerStatus> {
    let status = context.player.toggle_play()?;
    Ok(status)
}

pub fn player_get_volume(context: &mut AppContext) -> usize {
    context.player.get_volume()
}

pub fn player_set_volume(context: &mut AppContext, volume: usize) -> AppResult {
    context.player.set_volume(volume)?;
    Ok(())
}

pub fn player_volume_increase(context: &mut AppContext, amount: usize) -> AppResult<usize> {
    let volume = player_get_volume(context);

    let volume = if volume + amount > 100 {
        100
    } else {
        volume + amount
    };
    player_set_volume(context, volume)?;

    tracing::debug!(volume, "New volume level");
    Ok(volume)
}

pub fn player_volume_decrease(context: &mut AppContext, amount: usize) -> AppResult<usize> {
    let volume = player_get_volume(context);

    let volume = volume.saturating_sub(amount);
    player_set_volume(context, volume)?;

    tracing::debug!(volume, "New volume level");
    Ok(volume)
}

pub fn player_play_again(context: &mut AppContext) -> AppResult {
    context.player.play_again()?;
    run_on_song_change(context);
    Ok(())
}

pub fn player_play_next(context: &mut AppContext) -> AppResult {
    context.player.play_next()?;
    run_on_song_change(context);
    Ok(())
}

pub fn player_play_previous(context: &mut AppContext) -> AppResult {
    context.player.play_previous()?;
    run_on_song_change(context);
    Ok(())
}
