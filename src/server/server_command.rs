use dizi_lib::error::DiziResult;
use dizi_lib::player::PlayerStatus;
use dizi_lib::song::Song;

use crate::context::AppContext;
use crate::events::{ClientRequest, Events, ServerBroadcastEvent};
use crate::server_commands::*;

pub fn run_command(context: &mut AppContext, event: ClientRequest) -> DiziResult<()> {
    match event {
        ClientRequest::PlayerPlay(path) => {
            let song = Song::new(path.as_path())?;
            player_play(context, song.file_path())?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerPlay(song));
        }
        ClientRequest::PlayerPause => {
            player_pause(context)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerPause);
        }
        ClientRequest::PlayerResume => {
            player_resume(context)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerResume);
        }
        ClientRequest::PlayerNextSong => {
            eprintln!(
                "Error: '{:?}' not implemented",
                ClientRequest::PlayerNextSong
            );
        }
        ClientRequest::PlayerPrevSong => {
            eprintln!(
                "Error: '{:?}' not implemented",
                ClientRequest::PlayerPrevSong
            );
        }
        ClientRequest::PlayerGetVolume => {
            eprintln!(
                "Error: '{:?}' not implemented",
                ClientRequest::PlayerGetVolume
            );
        }
        ClientRequest::PlayerVolumeUp(amount) => {
            let volume = player_volume_increase(context, amount)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerVolumeUpdate(volume));
        }
        ClientRequest::PlayerVolumeDown(amount) => {
            let volume = player_volume_decrease(context, amount)?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerVolumeUpdate(volume));
        }
        ClientRequest::PlayerTogglePlay => {
            let status = player_toggle_play(context)?;
            match status {
                PlayerStatus::Playing => {
                    context
                        .events
                        .broadcast_event(ServerBroadcastEvent::PlayerResume);
                }
                PlayerStatus::Paused => {
                    context
                        .events
                        .broadcast_event(ServerBroadcastEvent::PlayerPause);
                }
                _ => {
                    context
                        .events
                        .broadcast_event(ServerBroadcastEvent::PlayerPause);
                }
            }
        }
        ClientRequest::PlayerToggleNext => {}
        ClientRequest::PlayerToggleRepeat => {}
        ClientRequest::PlayerToggleShuffle => {}
        s => {
            eprintln!("Error: '{:?}' not implemented", s);
        }
    }
    Ok(())
}
