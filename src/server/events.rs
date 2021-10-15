use std::convert::From;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time;

use dizi_lib::song::Song;

#[derive(Debug)]
pub enum ClientRequest {
    // new client
    NewClient(UnixStream),

    // quit server
    Quit,

    // player requests
    PlayerFilePlay(PathBuf),

    PlayerPlayNext,
    PlayerPlayPrevious,

    PlayerPause,
    PlayerResume,
    PlayerNextSong,
    PlayerPrevSong,
    PlayerGetVolume,

    PlayerVolumeUp(usize),
    PlayerVolumeDown(usize),

    PlayerTogglePlay,
    PlayerToggleNext,
    PlayerToggleRepeat,
    PlayerToggleShuffle,

    PlaylistOpen(PathBuf),
    PlaylistPlay(usize),

    PlaylistAppend(PathBuf),
    PlaylistRemove(usize),
    PlaylistMoveUp(usize),
    PlaylistMoveDown(usize),
}

#[derive(Clone, Debug)]
pub enum ServerEvent {
    PlayerProgressUpdate(time::Duration),
    PlayerDone,
}

#[derive(Debug)]
pub enum AppEvent {
    Server(ServerEvent),
    Client(ClientRequest),
}

#[derive(Clone, Debug)]
pub enum ServerBroadcastEvent {
    // server is shutting down
    Quit,

    // player status updates
    PlayerFilePlay(Song),

    PlayerPause,
    PlayerResume,

    PlayerRepeat(bool),
    PlayerShuffle(bool),
    PlayerNext(bool),

    PlayerVolumeUpdate(usize),
    PlayerProgressUpdate(time::Duration),

    PlaylistPlay(usize),
}

pub type AppEventSender = mpsc::Sender<AppEvent>;
pub type AppEventReceiver = mpsc::Receiver<AppEvent>;

pub type ClientRequestSender = mpsc::Sender<ClientRequest>;
pub type ClientRequestReceiver = mpsc::Receiver<ClientRequest>;

pub type ServerEventSender = mpsc::Sender<ServerEvent>;
pub type ServerEventReceiver = mpsc::Receiver<ServerEvent>;

pub type ServerBroadcastEventSender = mpsc::Sender<ServerBroadcastEvent>;
pub type ServerBroadcastEventReceiver = mpsc::Receiver<ServerBroadcastEvent>;

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`

#[derive(Debug)]
pub struct Events {
    // use if you want to send client requests
    pub client_request_tx: ClientRequestSender,
    // use if you want to send server events
    pub server_event_tx: ServerEventSender,

    // main listening loop
    pub app_event_tx: AppEventSender,
    pub app_event_rx: AppEventReceiver,

    pub server_broadcast_listeners: Vec<ServerBroadcastEventSender>,
}

impl Events {
    pub fn new() -> Self {
        Events::_new()
    }

    fn _new() -> Self {
        let (client_request_tx, client_request_rx) = mpsc::channel();
        let (server_event_tx, server_event_rx) = mpsc::channel();

        let (app_event_tx, app_event_rx) = mpsc::channel();

        // listen to client requests
        let app_event_tx2 = app_event_tx.clone();
        let _ = thread::spawn(move || loop {
            if let Ok(msg) = client_request_rx.recv() {
                app_event_tx2.send(AppEvent::Client(msg));
            }
        });

        // listen to server requests
        let app_event_tx2 = app_event_tx.clone();
        let _ = thread::spawn(move || loop {
            if let Ok(msg) = server_event_rx.recv() {
                app_event_tx2.send(AppEvent::Server(msg));
            }
        });

        Events {
            client_request_tx,
            server_event_tx,
            app_event_tx,
            app_event_rx,
            server_broadcast_listeners: Vec::with_capacity(4),
        }
    }

    pub fn client_request_sender(&self) -> &ClientRequestSender {
        &self.client_request_tx
    }

    pub fn server_event_sender(&self) -> &ServerEventSender {
        &self.server_event_tx
    }

    pub fn next(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.app_event_rx.recv()
    }

    pub fn add_broadcast_listener(&mut self, server_tx: ServerBroadcastEventSender) {
        self.server_broadcast_listeners.push(server_tx);
    }

    pub fn broadcast_event(&mut self, event: ServerBroadcastEvent) {
        let mut queue = Vec::with_capacity(self.server_broadcast_listeners.len());
        for server_tx in self.server_broadcast_listeners.iter_mut() {
            if server_tx.send(event.clone()).is_ok() {
                queue.push(server_tx.clone());
            }
        }
        self.server_broadcast_listeners = queue;
    }
}
