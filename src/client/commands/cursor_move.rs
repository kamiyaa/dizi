use dizi_lib::error::DiziResult;

use crate::config::option::WidgetType;
use crate::context::AppContext;
use crate::ui::TuiBackend;

pub fn safe_subtract(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        0
    }
}

pub fn cursor_move(context: &mut AppContext, widget: WidgetType, new_index: usize) {
    match widget {
        WidgetType::FileBrowser => set_curr_dirlist_index(context, new_index),
        WidgetType::Playlist => set_playlist_index(context, new_index),
        _ => {}
    }
}

pub fn cursor_index(context: &mut AppContext, widget: WidgetType) -> Option<usize> {
    match widget {
        WidgetType::FileBrowser => get_curr_dirlist_index(context),
        WidgetType::Playlist => get_playlist_index(context),
        _ => None,
    }
}

fn get_curr_dirlist_index(context: &AppContext) -> Option<usize> {
    context.curr_list_ref().and_then(|list| list.index)
}
fn get_curr_dirlist_len(context: &AppContext) -> Option<usize> {
    context.curr_list_ref().map(|list| list.len())
}
fn set_curr_dirlist_index(context: &mut AppContext, new_index: usize) {
    let mut new_index = new_index;
    if let Some(curr_list) = context.curr_list_mut() {
        if curr_list.is_empty() {
            return;
        }
        let dir_len = curr_list.len();
        if dir_len <= new_index {
            curr_list.index = Some(safe_subtract(dir_len, 1));
        } else {
            curr_list.index = Some(new_index);
        }
    }
}

fn get_playlist_index(context: &AppContext) -> Option<usize> {
    context
        .server_state_ref()
        .player_state_ref()
        .playlist_ref()
        .get_cursor_index()
}
fn get_playlist_len(context: &AppContext) -> usize {
    context
        .server_state_ref()
        .player_state_ref()
        .playlist_ref()
        .len()
}
fn set_playlist_index(context: &mut AppContext, new_index: usize) {
    let playlist_len = context
        .server_state_ref()
        .player_state_ref()
        .playlist_ref()
        .len();
    if playlist_len <= new_index {
        context
            .server_state_mut()
            .player_state_mut()
            .playlist_mut()
            .set_cursor_index(Some(safe_subtract(playlist_len, 1)));
    } else {
        context
            .server_state_mut()
            .player_state_mut()
            .playlist_mut()
            .set_cursor_index(Some(new_index));
    }
}

pub fn up(context: &mut AppContext, u: usize) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    if let Some(index) = index {
        let new_index = safe_subtract(index, u);
        cursor_move(context, widget, new_index);
    }
    Ok(())
}

pub fn down(context: &mut AppContext, u: usize) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    if let Some(index) = index {
        let new_index = index + u;
        cursor_move(context, widget, new_index);
    }
    Ok(())
}

pub fn home(context: &mut AppContext) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    match index {
        Some(index) if index > 0 => {
            cursor_move(context, widget, 0);
        }
        _ => {}
    }
    Ok(())
}

pub fn end(context: &mut AppContext) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = match widget {
        WidgetType::FileBrowser => get_curr_dirlist_index(context),
        WidgetType::Playlist => get_playlist_index(context),
        _ => None,
    };

    let len = match widget {
        WidgetType::FileBrowser => get_curr_dirlist_len(context),
        WidgetType::Playlist => Some(get_playlist_len(context)),
        _ => None,
    };

    match (index, len) {
        (Some(index), Some(len)) if index < len - 1 => {
            cursor_move(context, widget, len - 1);
        }
        _ => {}
    }
    Ok(())
}

fn get_page_size(context: &AppContext, backend: &TuiBackend) -> Option<usize> {
    Some(10)
}

pub fn page_up(context: &mut AppContext, backend: &mut TuiBackend) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    let page_size = get_page_size(context, backend).unwrap_or(10);

    if let Some(index) = index {
        let new_index = safe_subtract(index, page_size);
        cursor_move(context, widget, new_index);
    }
    Ok(())
}

pub fn page_down(context: &mut AppContext, backend: &mut TuiBackend) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    let page_size = get_page_size(context, backend).unwrap_or(10);

    if let Some(index) = index {
        let new_index = index + page_size;
        cursor_move(context, widget, new_index);
    }
    Ok(())
}
