use std::io;
use std::os::unix::net::UnixStream;
use std::path;
use std::sync::mpsc;
use std::thread;

#[derive(Debug)]
pub enum AppEvent {
    NewClient(UnixStream),
    Quit,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    pub event_tx: mpsc::Sender<AppEvent>,
    pub event_rx: mpsc::Receiver<AppEvent>,
}

impl Events {
    pub fn new() -> Self {
        Events::_new()
    }

    fn _new() -> Self {
        let (event_tx, event_rx) = mpsc::channel();

        Events { event_tx, event_rx }
    }

    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        let event = self.event_rx.recv()?;
        Ok(event)
    }
}
