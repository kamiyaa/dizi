use dizi_lib::error::DiziResult;
use dizi_lib::request::client::ClientRequest;

use crate::config::option::WidgetType;
use crate::context::AppContext;
use crate::util::request::send_client_request;

use super::change_directory;

pub fn open(context: &mut AppContext) -> DiziResult<()> {
    let widget = context.get_view_widget();

    match widget {
        WidgetType::FileBrowser => file_browser_open(context)?,
        WidgetType::Playlist => playlist_open(context)?,
        _ => {}
    }
    Ok(())
}

pub fn file_browser_open(context: &mut AppContext) -> DiziResult<()> {
    if let Some(entry) = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|s| s.curr_entry_ref())
    {
        if entry.file_path().is_dir() {
            let path = entry.file_path().to_path_buf();
            change_directory::cd(path.as_path(), context)?;
        } else {
            match entry.file_path().extension() {
                Some(s) => {
                    let s = s.to_string_lossy();
                    if s.as_ref().starts_with("m3u") {
                        let cwd = context.tab_context_ref().curr_tab_ref().cwd().to_path_buf();
                        let request = ClientRequest::PlaylistOpen {
                            cwd: Some(cwd),
                            path: Some(entry.file_path().to_path_buf()),
                        };
                        send_client_request(context, &request)?;
                    } else {
                        let request = ClientRequest::PlayerFilePlay {
                            path: Some(entry.file_path().to_path_buf()),
                        };
                        send_client_request(context, &request)?;
                    }
                }
                None => {
                    let request = ClientRequest::PlayerFilePlay {
                        path: Some(entry.file_path().to_path_buf()),
                    };
                    send_client_request(context, &request)?;
                }
            }
        }
    }
    Ok(())
}

pub fn playlist_open(context: &mut AppContext) -> DiziResult<()> {
    if let Some(index) = context
        .server_state_ref()
        .player_ref()
        .playlist_ref()
        .get_cursor_index()
    {
        let request = ClientRequest::PlaylistPlay { index: Some(index) };
        send_client_request(context, &request)?;
    }
    Ok(())
}
