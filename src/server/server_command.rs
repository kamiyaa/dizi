use dizi_lib::error::DiziResult;
use dizi_lib::player::PlayerStatus;
use dizi_lib::song::Song;

use crate::context::AppContext;
use crate::events::{ClientRequest, ServerBroadcastEvent};
use crate::server_commands::*;

pub fn run_command(context: &mut AppContext, event: ClientRequest) -> DiziResult<()> {
    match event {
        ClientRequest::PlayerFilePlay(path) => {
            let song = Song::new(path.as_path())?;
            player_play(context, song.file_path())?;
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerFilePlay(song));
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
        ClientRequest::PlayerPlayNext => {
            player_play_next(context)?;
        }
        ClientRequest::PlayerPlayPrevious => {
            player_play_previous(context)?;
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
        ClientRequest::PlayerToggleNext => {
            let next = !context.player_context_ref().player_ref().next_enabled();
            context.player_context_mut().player_mut().set_next(next);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerNext(next));
        }
        ClientRequest::PlayerToggleRepeat => {
            let repeat = !context.player_context_ref().player_ref().repeat_enabled();
            context.player_context_mut().player_mut().set_repeat(repeat);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerRepeat(repeat));
        }
        ClientRequest::PlayerToggleShuffle => {
            let shuffle = !context.player_context_ref().player_ref().shuffle_enabled();
            context
                .player_context_mut()
                .player_mut()
                .set_shuffle(shuffle);
            context
                .events
                .broadcast_event(ServerBroadcastEvent::PlayerShuffle(shuffle));
        }
        s => {
            eprintln!("Error: '{:?}' not implemented", s);
        }
    }
    Ok(())
}
