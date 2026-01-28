use std::sync::mpsc;
use std::thread;

use dizi::error::DiziResult;

use crate::audio::request::PlayerRequest;
use crate::audio::symphonia::stream::StreamEvent;

/// Combined enum
#[derive(Clone, Debug)]
pub enum PlayerStreamEvent {
    Stream(StreamEvent),
    Player(PlayerRequest),
}

/// Listen for events and wraps them into PlayerStreamEvent
#[derive(Debug)]
pub struct PlayerStreamEventListener {
    pub stream_tx: mpsc::Sender<StreamEvent>,
    pub player_res_tx: mpsc::Sender<DiziResult>,
    pub _event_tx: mpsc::Sender<PlayerStreamEvent>,
    pub event_rx: mpsc::Receiver<PlayerStreamEvent>,
}

impl PlayerStreamEventListener {
    pub fn new(
        player_res_tx: mpsc::Sender<DiziResult>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
    ) -> Self {
        Self::init(player_res_tx, player_req_rx)
    }

    fn init(
        player_res_tx: mpsc::Sender<DiziResult>,
        player_req_rx: mpsc::Receiver<PlayerRequest>,
    ) -> Self {
        let (stream_tx, stream_rx) = mpsc::channel();
        let (event_tx, event_rx) = mpsc::channel();

        // Listening for stream events
        let event_tx_clone = event_tx.clone();
        let _ = thread::spawn(move || {
            while let Ok(event) = stream_rx.recv() {
                let _ = event_tx_clone.send(PlayerStreamEvent::Stream(event));
            }
        });

        // Listening for user requests
        let event_tx_clone = event_tx.clone();
        let _ = thread::spawn(move || {
            while let Ok(req) = player_req_rx.recv() {
                let _ = event_tx_clone.send(PlayerStreamEvent::Player(req));
            }
        });

        Self {
            stream_tx,
            player_res_tx,
            _event_tx: event_tx,
            event_rx,
        }
    }

    pub fn next(&mut self) -> DiziResult<PlayerStreamEvent> {
        Ok(self.event_rx.recv()?)
    }

    pub fn player_res(&self) -> &mpsc::Sender<DiziResult> {
        &self.player_res_tx
    }
}
