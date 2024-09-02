use std::time::Duration;

use dizi::song::DiziAudioFile;

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play { song: DiziAudioFile, volume: f32 },
    Pause,
    Resume,
    Stop,
    SetVolume { volume: f32 },
    FastForward { offset: Duration },
    Rewind { offset: Duration },
    //    AddListener(ServerEventSender),
    //    ClearListeners,
}
