use dizi_lib::error::DiziResult;

use crate::config::option::WidgetType;
use crate::context::AppContext;
use crate::ui::AppBackend;

pub fn safe_subtract(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        0
    }
}

pub fn cursor_move(context: &mut AppContext, new_index: usize) {
    let widget = context.get_view_widget();
    cursor_move_for_widget(context, widget, new_index);
}

pub fn cursor_move_for_widget(context: &mut AppContext, widget: WidgetType, new_index: usize) {
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
    context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|list| list.get_index())
}
fn get_curr_dirlist_len(context: &AppContext) -> Option<usize> {
    context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|list| list.len())
}
fn set_curr_dirlist_index(context: &mut AppContext, new_index: usize) {
    let ui_context = context.ui_context_ref().clone();
    let display_options = context.config_ref().display_options_ref().clone();

    let new_index = new_index;
    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        if curr_list.is_empty() {
            return;
        }
        let dir_len = curr_list.len();
        if dir_len <= new_index {
            curr_list.set_index(
                Some(safe_subtract(dir_len, 1)),
                &ui_context,
                &display_options,
            );
        } else {
            curr_list.set_index(Some(new_index), &ui_context, &display_options);
        }
    }
}

fn get_playlist_index(context: &AppContext) -> Option<usize> {
    context
        .server_state_ref()
        .player_ref()
        .playlist_ref()
        .get_cursor_index()
}
fn get_playlist_len(context: &AppContext) -> usize {
    context.server_state_ref().player_ref().playlist_ref().len()
}
fn set_playlist_index(context: &mut AppContext, new_index: usize) {
    let playlist_len = context.server_state_ref().player_ref().playlist_ref().len();
    if playlist_len <= new_index {
        context
            .server_state_mut()
            .player_mut()
            .playlist_mut()
            .set_cursor_index(Some(safe_subtract(playlist_len, 1)));
    } else {
        context
            .server_state_mut()
            .player_mut()
            .playlist_mut()
            .set_cursor_index(Some(new_index));
    }
}

pub fn up(context: &mut AppContext, u: usize) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    if let Some(index) = index {
        let new_index = safe_subtract(index, u);
        cursor_move_for_widget(context, widget, new_index);
    }
    Ok(())
}

pub fn down(context: &mut AppContext, u: usize) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    if let Some(index) = index {
        let new_index = index + u;
        cursor_move_for_widget(context, widget, new_index);
    }
    Ok(())
}

pub fn home(context: &mut AppContext) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    match index {
        Some(index) if index > 0 => {
            cursor_move_for_widget(context, widget, 0);
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
            cursor_move_for_widget(context, widget, len - 1);
        }
        _ => {}
    }
    Ok(())
}

fn get_page_size(_context: &AppContext, _backend: &AppBackend) -> Option<usize> {
    Some(10)
}

pub fn page_up(context: &mut AppContext, backend: &mut AppBackend) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    let page_size = get_page_size(context, backend).unwrap_or(10);

    if let Some(index) = index {
        let new_index = safe_subtract(index, page_size);
        cursor_move_for_widget(context, widget, new_index);
    }
    Ok(())
}

pub fn page_down(context: &mut AppContext, backend: &mut AppBackend) -> DiziResult<()> {
    let widget = context.get_view_widget();
    let index = cursor_index(context, widget);

    let page_size = get_page_size(context, backend).unwrap_or(10);

    if let Some(index) = index {
        let new_index = index + page_size;
        cursor_move_for_widget(context, widget, new_index);
    }
    Ok(())
}
