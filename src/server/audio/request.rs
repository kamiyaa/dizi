use std::time::Duration;

use dizi_lib::song::Song;

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play(Song),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
    FastForward(Duration),
    Rewind(Duration),
    //    AddListener(ServerEventSender),
    //    ClearListeners,
}
