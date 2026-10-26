use ratatui::layout::Rect;
use std::io;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::mpsc;

use dizi::utils;

use crate::config;
use crate::config::option::WidgetType;
use crate::context::{MessageQueue, ServerState, TabState};
use crate::event::{AppEvent, AppEventListener};
use crate::state::CommandLineState;
use crate::utils::search::SearchPattern;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuitType {
    DoNot,
    Normal,
    Server,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiState {
    pub layout: Vec<Rect>,
}

pub struct AppState {
    pub quit: QuitType,
    // event loop querying
    pub event_listener: AppEventListener,
    // server unix socket
    pub stream: UnixStream,
    pub view_widget: WidgetType,
    // app config
    pub config: config::AppConfig,

    // context related to tabs
    pub tab_state: TabState,

    pub commandline_state: CommandLineState,
    // user interface context; data which is input to both, the UI rendering and the app state
    pub ui_state: UiState,
    // context related to searching
    search_context: Option<SearchPattern>,
    // message queue for displaying messages
    pub message_queue: MessageQueue,
    // server state
    pub server_state: ServerState,
}

impl AppState {
    pub fn new(config: config::AppConfig, _cwd: PathBuf, stream: UnixStream) -> Self {
        let events = AppEventListener::new();

        let commandline_state = CommandLineState::new();
        Self {
            quit: QuitType::DoNot,
            config,
            stream,
            view_widget: WidgetType::FileBrowser,
            event_listener: events,
            commandline_state,
            search_context: None,
            tab_state: TabState::new(),
            ui_state: UiState { layout: vec![] },
            message_queue: MessageQueue::new(),
            server_state: ServerState::new(),
        }
    }

    pub fn clone_stream(&self) -> io::Result<UnixStream> {
        self.stream.try_clone()
    }

    pub fn flush_stream(&mut self) -> io::Result<()> {
        utils::flush(&mut self.stream)
    }

    // event related
    pub fn poll_event(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.event_listener.next()
    }
    pub fn flush_event(&self) {
        self.event_listener.flush();
    }
    pub fn clone_event_tx(&self) -> mpsc::Sender<AppEvent> {
        self.event_listener.event_tx.clone()
    }

    pub fn get_search_context(&self) -> Option<&SearchPattern> {
        self.search_context.as_ref()
    }
    pub fn set_search_context(&mut self, pattern: SearchPattern) {
        self.search_context = Some(pattern);
    }

    pub fn get_view_widget(&self) -> WidgetType {
        self.view_widget
    }
    pub fn set_view_widget(&mut self, widget: WidgetType) {
        self.view_widget = widget;
    }
}
