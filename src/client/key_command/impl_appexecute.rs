use dizi_lib::error::DiziResult;
use dizi_lib::request::client::ClientRequest;

use crate::commands::*;
use crate::config::option::WidgetType;
use crate::config::AppKeyMapping;
use crate::context::AppContext;
use crate::ui::AppBackend;
use crate::util::request::send_client_request;

use super::{AppExecute, Command};

impl AppExecute for Command {
    fn execute(
        &self,
        context: &mut AppContext,
        backend: &mut AppBackend,
        keymap_t: &AppKeyMapping,
    ) -> DiziResult {
        match self {
            Self::ChangeDirectory(p) => {
                change_directory::change_directory(context, p.as_path())?;
            }
            Self::CommandLine(p, s) => {
                command_line::read_and_execute(context, backend, keymap_t, p.as_str(), s.as_str())?
            }

            Self::CursorMoveUp(u) => cursor_move::up(context, *u)?,
            Self::CursorMoveDown(u) => cursor_move::down(context, *u)?,
            Self::CursorMoveHome => cursor_move::home(context)?,
            Self::CursorMoveEnd => cursor_move::end(context)?,
            Self::CursorMovePageUp => cursor_move::page_up(context, backend)?,
            Self::CursorMovePageDown => cursor_move::page_down(context, backend)?,

            Self::GoToPlaying => goto::goto_playing(context)?,

            Self::ParentDirectory => change_directory::parent_directory(context)?,

            Self::Close => quit::close(context)?,

            Self::ReloadDirList => reload::reload_dirlist(context)?,

            Self::SearchGlob(pattern) => search_glob::search_glob(context, pattern.as_str())?,
            Self::SearchString(pattern) => search_string::search_string(context, pattern.as_str())?,
            Self::SearchSkim => search_skim::search_skim(context, backend)?,
            Self::SearchNext => search::search_next(context)?,
            Self::SearchPrev => search::search_prev(context)?,

            Self::SelectFiles(pattern, options) => {
                selection::select_files(context, pattern.as_str(), options)?
            }

            Self::ServerRequest(request) => execute_request(context, request)?,

            Self::ToggleHiddenFiles => show_hidden::toggle_hidden(context)?,
            Self::ToggleView => {
                let new_widget = match context.get_view_widget() {
                    WidgetType::FileBrowser => WidgetType::Playlist,
                    WidgetType::Playlist => WidgetType::FileBrowser,
                    s => s,
                };
                context.set_view_widget(new_widget);
            }
            Self::Sort(t) => sort::set_sort(context, *t)?,
            Self::SortReverse => sort::toggle_reverse(context)?,

            Self::OpenFile => open_file::open(context)?,
        }
        Ok(())
    }
}

pub fn execute_request(context: &mut AppContext, request: &ClientRequest) -> DiziResult {
    match request {
        ClientRequest::ServerQuit => {
            quit::server_quit(context)?;
        }
        ClientRequest::PlaylistAppend { path: None } => {
            if let Some(entry) = context
                .tab_context_ref()
                .curr_tab_ref()
                .curr_list_ref()
                .and_then(|s| s.curr_entry_ref())
            {
                let request = ClientRequest::PlaylistAppend {
                    path: Some(entry.file_path().to_path_buf()),
                };
                send_client_request(context, &request)?;
            }
        }
        ClientRequest::PlaylistOpen {
            cwd: None,
            path: None,
        } => {
            if let Some(entry) = context
                .tab_context_ref()
                .curr_tab_ref()
                .curr_list_ref()
                .and_then(|s| s.curr_entry_ref())
            {
                let cwd = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
                let request = ClientRequest::PlaylistOpen {
                    cwd: Some(cwd),
                    path: Some(entry.file_path().to_path_buf()),
                };
                send_client_request(context, &request)?;
            }
        }
        ClientRequest::PlaylistPlay { index: None } => {
            let playlist = context.server_state_ref().player_ref().playlist_ref();
            if let Some(index) = playlist.get_cursor_index() {
                let request = ClientRequest::PlaylistPlay { index: Some(index) };
                send_client_request(context, &request)?;
            }
        }
        ClientRequest::PlaylistRemove { index: None } => {
            if context.get_view_widget() != WidgetType::Playlist {
                return Ok(());
            }
            let playlist = context.server_state_ref().player_ref().playlist_ref();
            if let Some(index) = playlist.get_cursor_index() {
                let request = ClientRequest::PlaylistRemove { index: Some(index) };
                send_client_request(context, &request)?;
            }
        }
        ClientRequest::PlaylistMoveUp { index: None } => {
            if context.get_view_widget() != WidgetType::Playlist {
                return Ok(());
            }
            let playlist = context.server_state_ref().player_ref().playlist_ref();
            if let Some(index) = playlist.get_cursor_index() {
                let request = ClientRequest::PlaylistMoveUp { index: Some(index) };
                send_client_request(context, &request)?;
            }
        }
        ClientRequest::PlaylistMoveDown { index: None } => {
            if context.get_view_widget() != WidgetType::Playlist {
                return Ok(());
            }
            let playlist = context.server_state_ref().player_ref().playlist_ref();
            if let Some(index) = playlist.get_cursor_index() {
                let request = ClientRequest::PlaylistMoveDown { index: Some(index) };
                send_client_request(context, &request)?;
            }
        }
        request => {
            send_client_request(context, request)?;
        }
    }
    Ok(())
}
