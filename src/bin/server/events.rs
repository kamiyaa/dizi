use std::collections::HashMap;
use std::os::unix::net::UnixStream;
use std::sync::mpsc;
use std::time;

use dizi::request::client::ClientRequest;
use dizi::response::server::ServerBroadcastEvent;

use crate::client::ClientMessage;

#[derive(Debug)]
pub enum ServerEvent {
    // new client connected
    NewClient(UnixStream),
    /// player duration update
    PlayerProgressUpdate(time::Duration),
    /// song is done
    PlayerDone,
}

#[derive(Debug)]
pub enum AppEvent {
    Server {
        event: ServerEvent,
    },
    Client {
        uuid: String,
        request: ClientRequest,
    },
}

impl From<ServerEvent> for AppEvent {
    fn from(event: ServerEvent) -> Self {
        Self::Server { event }
    }
}

/// Every thread sends events for the main loop over clones of this sender
pub type AppEventSender = mpsc::Sender<AppEvent>;
pub type AppEventReceiver = mpsc::Receiver<AppEvent>;

/// Send messages to a connected client's thread
pub type ClientMessageSender = mpsc::Sender<ClientMessage>;

#[derive(Debug)]
pub struct Events {
    // clone this to send events to the main loop
    pub app_event_tx: AppEventSender,
    // main listening loop
    pub app_event_rx: AppEventReceiver,

    pub server_broadcast_listeners: HashMap<String, ClientMessageSender>,
}

impl Events {
    pub fn new() -> Self {
        let (app_event_tx, app_event_rx) = mpsc::channel();
        Events {
            app_event_tx,
            app_event_rx,
            server_broadcast_listeners: HashMap::new(),
        }
    }

    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.app_event_rx.recv()
    }

    pub fn add_broadcast_listener(&mut self, uuid: String, client_tx: ClientMessageSender) {
        self.server_broadcast_listeners.insert(uuid, client_tx);
    }

    pub fn broadcast_event(&mut self, event: ServerBroadcastEvent) {
        match &event {
            ServerBroadcastEvent::PlayerState { .. } => {}
            event => {
                tracing::debug!(
                    ?event,
                    client_count = self.server_broadcast_listeners.len(),
                    "Broadcasting event to all clients",
                );
            }
        }
        for client_tx in self.server_broadcast_listeners.values() {
            let _ = client_tx.send(ClientMessage::Server(Box::new(event.clone())));
        }
    }
}
