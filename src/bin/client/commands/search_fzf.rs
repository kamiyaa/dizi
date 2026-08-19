use dizi::error::{AppResult, DiziError, DiziErrorKind};

use crate::commands::{cursor_move, fzf};
use crate::config::option::WidgetType;
use crate::context::AppState;
use crate::ui::AppBackend;

pub fn search_fzf(app_state: &mut AppState, backend: &mut AppBackend) -> AppResult {
    let widget = app_state.get_view_widget();

    match widget {
        WidgetType::FileBrowser => fzf_search_directory(app_state, backend, widget)?,
        WidgetType::Playlist => fzf_search_playlist(app_state, backend, widget)?,
        _ => {}
    }
    Ok(())
}

fn fzf_search_playlist(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    widget: WidgetType,
) -> AppResult {
    let items: Vec<String> = app_state
        .server_state_ref()
        .player
        .playlist
        .list_ref()
        .iter()
        .enumerate()
        .map(|(i, entry)| format!("{:04} \u{02503} {}\n", i + 1, entry.file_name()))
        .collect();

    if items.is_empty() {
        return Err(DiziError::new(
            DiziErrorKind::ParseError,
            "no files to select".to_string(),
        ));
    }

    let fzf_output = fzf::fzf(app_state, backend, items)?;
    let selected_idx_str = fzf_output.split_once(' ');

    if let Some((selected_idx_str, _)) = selected_idx_str {
        if let Ok(index) = selected_idx_str.parse::<usize>() {
            let index = index.saturating_sub(1);
            cursor_move::cursor_move_for_widget(app_state, widget, index);
        }
    }
    Ok(())
}

fn fzf_search_directory(
    app_state: &mut AppState,
    backend: &mut AppBackend,
    widget: WidgetType,
) -> AppResult {
    let items = app_state
        .tab_state_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|list| {
            let v: Vec<String> = list
                .iter()
                .enumerate()
                .map(|(i, entry)| format!("{:04} \u{02503} {}\n", i + 1, entry.file_name()))
                .collect();
            v
        })
        .unwrap_or_default();

    if items.is_empty() {
        return Err(DiziError::new(
            DiziErrorKind::ParseError,
            "no files to select".to_string(),
        ));
    }

    let fzf_output = fzf::fzf(app_state, backend, items)?;
    let selected_idx_str = fzf_output.split_once(' ');

    if let Some((selected_idx_str, _)) = selected_idx_str {
        if let Ok(index) = selected_idx_str.parse::<usize>() {
            let index = index.saturating_sub(1);
            cursor_move::cursor_move_for_widget(app_state, widget, index);
        }
    }
    Ok(())
}
