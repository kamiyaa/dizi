use dizi_lib::error::DiziResult;

use crate::audio::PlayerStatus;
use crate::context::AppContext;
use crate::events::{ClientEvent, Events, ServerEvent};
use crate::server_commands::*;

pub fn run_command(
    context: &mut AppContext,
    events: &mut Events,
    event: ClientEvent,
) -> DiziResult<()> {
    match event {
        ClientEvent::PlayerGetLen => {
            let len = player_get_len(context)?;
            events.broadcast_event(ServerEvent::PlayerDurationLeft(len));
        }
        ClientEvent::PlayerPlay(song) => {
            player_play(context, song.file_path())?;
            events.broadcast_event(ServerEvent::PlayerPlay(song));
        }
        ClientEvent::PlayerPause => {
            player_pause(context)?;
            events.broadcast_event(ServerEvent::PlayerPause);
        }
        ClientEvent::PlayerResume => {
            player_resume(context)?;
            events.broadcast_event(ServerEvent::PlayerResume);
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
            events.broadcast_event(ServerEvent::PlayerVolumeUpdate(volume));
        }
        ClientEvent::PlayerVolumeDown(amount) => {
            let volume = player_volume_decrease(context, amount)?;
            events.broadcast_event(ServerEvent::PlayerVolumeUpdate(volume));
        }
        ClientEvent::PlayerTogglePlay => {
            let status = player_toggle_play(context)?;
            match status {
                PlayerStatus::Playing => {
                    events.broadcast_event(ServerEvent::PlayerResume);
                }
                PlayerStatus::Paused => {
                    events.broadcast_event(ServerEvent::PlayerPause);
                }
                _ => {
                    events.broadcast_event(ServerEvent::PlayerPause);
                }
            }
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
