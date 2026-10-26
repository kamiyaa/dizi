mod player_stream;
mod player_stream_state;

pub use player_stream::*;
pub use player_stream_state::*;

use std::time::Duration;

use crate::audio::request::PlayerRequest;

/// Events returned from stream
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamEvent {
    Progress(Duration),
    StreamEnded,
}

/// Everything the stream thread reacts to, sent over a single channel
#[derive(Clone, Debug)]
pub enum PlayerStreamEvent {
    /// From the output stream with the given id
    Stream {
        id: u64,
        event: StreamEvent,
    },
    Player(PlayerRequest),
}
