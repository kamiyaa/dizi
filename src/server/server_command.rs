use dizi_lib::error::DiziResult;

use crate::audio::PlayerStatus;
use crate::context::AppContext;
use crate::events::{ClientEvent, Events, ServerEvent};
use crate::server_commands::*;

pub fn run_command(context: &mut AppContext, event: ClientEvent) -> DiziResult<()> {
    match event {
        ClientEvent::PlayerPlay(song) => {
            player_play(context, song.file_path())?;
            context
                .events
                .broadcast_event(ServerEvent::PlayerPlay(song));
        }
        ClientEvent::PlayerPause => {
            player_pause(context)?;
            context.events.broadcast_event(ServerEvent::PlayerPause);
        }
        ClientEvent::PlayerResume => {
            player_resume(context)?;
            context.events.broadcast_event(ServerEvent::PlayerResume);
        }
        ClientEvent::PlayerNextSong => {
            eprintln!("Error: '{:?}' not implemented", ClientEvent::PlayerNextSong);
        }
        ClientEvent::PlayerPrevSong => {
            eprintln!("Error: '{:?}' not implemented", ClientEvent::PlayerPrevSong);
        }
        ClientEvent::PlayerGetVolume => {
            eprintln!(
                "Error: '{:?}' not implemented",
                ClientEvent::PlayerGetVolume
            );
        }
        ClientEvent::PlayerVolumeUp(amount) => {
            let volume = player_volume_increase(context, amount)?;
            context
                .events
                .broadcast_event(ServerEvent::PlayerVolumeUpdate(volume));
        }
        ClientEvent::PlayerVolumeDown(amount) => {
            let volume = player_volume_decrease(context, amount)?;
            context
                .events
                .broadcast_event(ServerEvent::PlayerVolumeUpdate(volume));
        }
        ClientEvent::PlayerTogglePlay => {
            let status = player_toggle_play(context)?;
            match status {
                PlayerStatus::Playing => {
                    context.events.broadcast_event(ServerEvent::PlayerResume);
                }
                PlayerStatus::Paused => {
                    context.events.broadcast_event(ServerEvent::PlayerPause);
                }
                _ => {
                    context.events.broadcast_event(ServerEvent::PlayerPause);
                }
            }
        }
        ClientEvent::PlayerProgressUpdate(t) => {
            context
                .events
                .broadcast_event(ServerEvent::PlayerProgressUpdate(t));
        }
        ClientEvent::PlayerToggleNext => {}
        ClientEvent::PlayerToggleRepeat => {}
        ClientEvent::PlayerToggleShuffle => {}
        s => {
            eprintln!("Error: '{:?}' not implemented", s);
        }
    }
    Ok(())
}
