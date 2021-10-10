use std::os::unix::net::UnixStream;
use std::sync::mpsc;

use dizi_lib::song::Song;

#[derive(Debug)]
pub enum ClientEvent {
    // new client
    NewClient(UnixStream),

    // quit server
    Quit,

    // player requests
    PlayerPlay(Song),
    PlayerPause,
    PlayerResume,
    PlayerNextSong,
    PlayerPrevSong,
    PlayerGetVolume,
    PlayerGetLen,
    PlayerVolumeUp(usize),
    PlayerVolumeDown(usize),
    PlayerTogglePlay,
    PlayerToggleNext,
    PlayerToggleRepeat,
    PlayerToggleShuffle,
}

#[derive(Clone, Debug)]
pub enum ServerEvent {
    // server is shutting down
    Quit,

    // player status updates
    PlayerPlay(Song),
    PlayerPause,
    PlayerResume,
    PlayerRepeatOn,
    PlayerRepeatOff,
    PlayerShuffleOn,
    PlayerShuffleOff,
    PlayerVolumeUpdate(usize),
    PlayerDurationLeft(usize),
}

pub type ClientEventSender = mpsc::Sender<ClientEvent>;
pub type ClientEventReceiver = mpsc::Receiver<ClientEvent>;

pub type ServerEventSender = mpsc::Sender<ServerEvent>;
pub type ServerEventReceiver = mpsc::Receiver<ServerEvent>;

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    pub client_tx: ClientEventSender,
    pub client_rx: ClientEventReceiver,
    pub server_listeners: Vec<ServerEventSender>,
}

impl Events {
    pub fn new() -> Self {
        Events::_new()
    }

    fn _new() -> Self {
        let (client_tx, client_rx) = mpsc::channel();

        Events {
            client_tx,
            client_rx,
            server_listeners: Vec::with_capacity(4),
        }
    }

    pub fn client_event_sender(&self) -> &ClientEventSender {
        &self.client_tx
    }

    pub fn client_event_receiver(&self) -> &ClientEventReceiver {
        &self.client_rx
    }

    pub fn next(&self) -> Result<ClientEvent, mpsc::RecvError> {
        let event = self.client_rx.recv()?;
        Ok(event)
    }

    pub fn add_listener(&mut self, server_tx: ServerEventSender) {
        self.server_listeners.push(server_tx);
    }

    pub fn broadcast_event(&mut self, event: ServerEvent) {
        let mut queue = Vec::with_capacity(self.server_listeners.len());
        for server_tx in self.server_listeners.iter_mut() {
            if server_tx.send(event.clone()).is_ok() {
                queue.push(server_tx.clone());
            }
        }
        self.server_listeners = queue;
    }
}
