use std::io;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::sync::mpsc;
use tui::layout::Rect;

use dizi_lib::utils;

use crate::config;
use crate::config::option::WidgetType;
use crate::context::{CommandLineContext, MessageQueue, ServerState, TabContext};
use crate::event::{AppEvent, Events};
use crate::util::search::SearchPattern;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QuitType {
    DoNot,
    Normal,
    Server,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiContext {
    pub layout: Vec<Rect>,
}

pub struct AppContext {
    pub quit: QuitType,
    // event loop querying
    pub events: Events,
    // server unix socket
    pub stream: UnixStream,
    pub view_widget: WidgetType,
    // app config
    config: config::AppConfig,

    _cwd: PathBuf,
    // context related to tabs
    tab_context: TabContext,

    commandline_context: CommandLineContext,
    // user interface context; data which is input to both, the UI rendering and the app state
    ui_context: UiContext,
    // context related to searching
    search_context: Option<SearchPattern>,
    // message queue for displaying messages
    message_queue: MessageQueue,
    // server state
    server_state: ServerState,
}

impl AppContext {
    pub fn new(config: config::AppConfig, cwd: PathBuf, stream: UnixStream) -> Self {
        let events = Events::new();

        let mut commandline_context = CommandLineContext::new();
        commandline_context.history_mut().set_max_len(20);

        Self {
            quit: QuitType::DoNot,
            config,
            stream,
            view_widget: WidgetType::FileBrowser,
            events,
            _cwd: cwd,
            commandline_context,
            search_context: None,
            tab_context: TabContext::new(),
            ui_context: UiContext { layout: vec![] },
            message_queue: MessageQueue::new(),
            server_state: ServerState::new(),
        }
    }

    pub fn clone_stream(&self) -> io::Result<UnixStream> {
        self.stream.try_clone()
    }

    pub fn flush_stream(&mut self) -> io::Result<()> {
        utils::flush(&mut self.stream)?;
        Ok(())
    }

    // event related
    pub fn poll_event(&self) -> Result<AppEvent, mpsc::RecvError> {
        self.events.next()
    }
    pub fn flush_event(&self) {
        self.events.flush();
    }
    pub fn clone_event_tx(&self) -> mpsc::Sender<AppEvent> {
        self.events.event_tx.clone()
    }

    pub fn config_ref(&self) -> &config::AppConfig {
        &self.config
    }
    pub fn config_mut(&mut self) -> &mut config::AppConfig {
        &mut self.config
    }

    pub fn message_queue_ref(&self) -> &MessageQueue {
        &self.message_queue
    }
    pub fn message_queue_mut(&mut self) -> &mut MessageQueue {
        &mut self.message_queue
    }

    pub fn server_state_ref(&self) -> &ServerState {
        &self.server_state
    }
    pub fn server_state_mut(&mut self) -> &mut ServerState {
        &mut self.server_state
    }

    pub fn tab_context_ref(&self) -> &TabContext {
        &self.tab_context
    }
    pub fn tab_context_mut(&mut self) -> &mut TabContext {
        &mut self.tab_context
    }

    pub fn get_search_context(&self) -> Option<&SearchPattern> {
        self.search_context.as_ref()
    }
    pub fn set_search_context(&mut self, pattern: SearchPattern) {
        self.search_context = Some(pattern);
    }

    pub fn ui_context_ref(&self) -> &UiContext {
        &self.ui_context
    }
    pub fn ui_context_mut(&mut self) -> &mut UiContext {
        &mut self.ui_context
    }

    pub fn commandline_context_ref(&self) -> &CommandLineContext {
        &self.commandline_context
    }
    pub fn commandline_context_mut(&mut self) -> &mut CommandLineContext {
        &mut self.commandline_context
    }

    pub fn get_view_widget(&self) -> WidgetType {
        self.view_widget
    }
    pub fn set_view_widget(&mut self, widget: WidgetType) {
        self.view_widget = widget;
    }
}
