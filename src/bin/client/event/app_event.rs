use std::io;
use std::path;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;

use ratatui::termion::event::Event;

use crate::event::input_listener::TerminalInputListener;
use crate::event::SignalListener;
use crate::fs::JoshutoDirList;

pub type AppEventSender = Sender<AppEvent>;

#[derive(Debug)]
pub enum AppEvent {
    TerminalEvent(Event),
    // preview thread events
    PreviewDir {
        path: path::PathBuf,
        res: Box<io::Result<JoshutoDirList>>,
    },
    Signal(i32),
    Server(String),
}

pub struct AppEventListener {
    pub event_tx: mpsc::Sender<AppEvent>,
    event_rx: mpsc::Receiver<AppEvent>,
    pub input_tx: mpsc::Sender<()>,
}

impl AppEventListener {
    pub fn new() -> Self {
        AppEventListener::with_config()
    }

    pub fn with_config() -> Self {
        let (input_tx, input_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        // signal thread
        let signal_listener = SignalListener::new(event_tx.clone());
        let _ = thread::spawn(move || {
            signal_listener.run();
        });

        // edge case that starts off the input thread
        let _ = input_tx.send(());
        // input thread
        let input_listener = TerminalInputListener::new(event_tx.clone(), input_rx);
        let _ = thread::spawn(move || {
            input_listener.run();
        });

        AppEventListener {
            event_tx,
            event_rx,
            input_tx,
        }
    }

    // We need a next() and a flush() so we don't continuously consume
    // input from the console. Sometimes, other applications need to
    // read terminal inputs while joshuto is in the background
    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        let event = self.event_rx.recv()?;
        Ok(event)
    }

    pub fn flush(&self) {
        loop {
            if self.input_tx.send(()).is_ok() {
                break;
            }
        }
    }
}
