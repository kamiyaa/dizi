use dizi::song::DiziAudioFile;

/// User requests to the player that need the audio stream itself. Volume and seeking are
/// changed through `PlaybackControl` instead, without a round trip through the stream thread.
#[derive(Clone, Debug)]
pub enum PlayerRequest {
    Play { song: Box<DiziAudioFile> },
    Pause,
    Resume,
    Stop,
}
