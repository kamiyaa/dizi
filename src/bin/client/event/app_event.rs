use std::io;
use std::path;
use std::sync::mpsc;
use std::thread;

use signal_hook::consts::signal;
use signal_hook::iterator::exfiltrator::SignalOnly;
use signal_hook::iterator::SignalsInfo;

use termion::event::Event;
use termion::input::TermRead;

use crate::fs::JoshutoDirList;

#[derive(Debug)]
pub enum AppEvent {
    Termion(Event),
    // preview thread events
    PreviewDir {
        path: path::PathBuf,
        res: Box<io::Result<JoshutoDirList>>,
    },
    Signal(i32),
    Server(String),
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Config {}

pub struct Events {
    pub event_tx: mpsc::Sender<AppEvent>,
    event_rx: mpsc::Receiver<AppEvent>,
    pub input_tx: mpsc::Sender<()>,
}

impl Events {
    pub fn new() -> Self {
        Events::with_config()
    }

    pub fn with_config() -> Self {
        let (input_tx, input_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        // edge case that starts off the input thread
        let _ = input_tx.send(());

        // signal thread
        let event_tx2 = event_tx.clone();
        let _ = thread::spawn(move || {
            let sigs = vec![signal::SIGWINCH];
            let mut signals = SignalsInfo::<SignalOnly>::new(&sigs).unwrap();
            for signal in &mut signals {
                if let Err(e) = event_tx2.send(AppEvent::Signal(signal)) {
                    eprintln!("Signal thread send err: {:#?}", e);
                    return;
                }
            }
        });

        // input thread
        let event_tx2 = event_tx.clone();
        let _ = thread::spawn(move || {
            let stdin = io::stdin();
            let mut events = stdin.events();

            loop {
                let _ = input_rx.recv();
                if let Some(Ok(event)) = events.next() {
                    let _ = event_tx2.send(AppEvent::Termion(event));
                }
            }
        });

        Events {
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
