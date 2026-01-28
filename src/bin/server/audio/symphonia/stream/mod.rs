mod player_stream;
mod player_stream_state;
mod stream_listener;

pub use player_stream::*;
pub use player_stream_state::*;
pub use stream_listener::*;

use std::time::Duration;

/// Events returned from stream
#[derive(Clone, Copy, Debug)]
pub enum StreamEvent {
    Progress(Duration),
    StreamEnded,
}
