use dizi_lib::song::Song;

#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play(Song),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
    //    AddListener(ServerEventSender),
    //    ClearListeners,
}
