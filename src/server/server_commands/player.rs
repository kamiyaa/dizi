use std::path::Path;

use dizi_lib::error::DiziResult;
use dizi_lib::player::PlayerStatus;
use dizi_lib::playlist::PlaylistStatus;
use dizi_lib::song::Song;

use crate::context::AppContext;

pub fn player_play(context: &mut AppContext, path: &Path) -> DiziResult<()> {
    context
        .player_context_mut()
        .player_mut()
        .play_entire_directory(path)
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

pub fn player_play_again(context: &mut AppContext) -> DiziResult<()> {
    let playlist_status = context.player_context_ref().player_ref().playlist_status();
    match playlist_status {
        PlaylistStatus::DirectoryListing => {
            let song = {
                let playlist = context
                    .player_context_ref()
                    .player_ref()
                    .dirlist_playlist_ref();
                let index = playlist.index;
                let len = playlist.len();
                let next_song_path = &playlist.list_ref()[index];
                Song::new(next_song_path)?
            };
            context.player_context_mut().player_mut().play(&song)?;
        }
        PlaylistStatus::PlaylistFile => {
            let index = context
                .player_context_ref()
                .player_ref()
                .playlist_ref()
                .get_playing_index();
            let len = context
                .player_context_ref()
                .player_ref()
                .playlist_ref()
                .len();
            if let Some(index) = index {
                context
                    .player_context_mut()
                    .player_mut()
                    .play_from_playlist(index)?;
            }
        }
    }
    Ok(())
}

pub fn player_play_next(context: &mut AppContext, skip_amount: usize) -> DiziResult<()> {
    let playlist_status = context.player_context_ref().player_ref().playlist_status();
    match playlist_status {
        PlaylistStatus::DirectoryListing => {
            let new_index = {
                let playlist = context
                    .player_context_ref()
                    .player_ref()
                    .dirlist_playlist_ref();
                let index = playlist.get_playing_index();
                let len = playlist.len();

                let new_index = if index + skip_amount >= len {
                    (index + skip_amount) % len
                } else {
                    index + skip_amount
                };
                new_index
            };
            context
                .player_context_mut()
                .player_mut()
                .play_from_directory(new_index)?;
        }
        PlaylistStatus::PlaylistFile => {
            let index = context
                .player_context_ref()
                .player_ref()
                .playlist_ref()
                .get_playing_index();
            let len = context
                .player_context_ref()
                .player_ref()
                .playlist_ref()
                .len();
            if let Some(index) = index {
                let new_index = if index + skip_amount >= len {
                    (index + skip_amount) % len
                } else {
                    index + skip_amount
                };
                context
                    .player_context_mut()
                    .player_mut()
                    .play_from_playlist(new_index)?;
            }
        }
    }
    Ok(())
}

pub fn player_play_previous(context: &mut AppContext) -> DiziResult<()> {
    let playlist_status = context.player_context_ref().player_ref().playlist_status();
    match playlist_status {
        PlaylistStatus::DirectoryListing => {
            let song = {
                let playlist = context
                    .player_context_ref()
                    .player_ref()
                    .dirlist_playlist_ref();
                let index = playlist.index;
                let len = playlist.len();

                let new_index = if index == 0 { len - 1 } else { index - 1 };
                let next_song_path = &playlist.list_ref()[new_index];

                Song::new(next_song_path)?
            };
            context.player_context_mut().player_mut().play(&song)?;
        }
        PlaylistStatus::PlaylistFile => {
            let index = context
                .player_context_ref()
                .player_ref()
                .playlist_ref()
                .get_playing_index();
            let len = context
                .player_context_ref()
                .player_ref()
                .playlist_ref()
                .len();
            if let Some(index) = index {
                let new_index = if index == 0 { len - 1 } else { index - 1 };
                context
                    .player_context_mut()
                    .player_mut()
                    .play_from_playlist(new_index)?;
            }
        }
    }
    Ok(())
}
