use std::borrow;
use std::io;
use std::sync;
use std::thread;

use skim::prelude::*;

use dizi::error::{DiziError, DiziErrorKind, DiziResult};

use crate::commands::cursor_move;
use crate::config::option::WidgetType;
use crate::context::AppContext;
use crate::ui::AppBackend;
use crate::utils::search::SearchPattern;

#[derive(Clone, Debug)]
pub struct DiziSkimItem {
    pub idx: usize,
    pub value: String,
}

impl SkimItem for DiziSkimItem {
    fn text<'a>(&'a self) -> Cow<'a, str> {
        borrow::Cow::Borrowed(self.value.as_str())
    }
}

pub fn search_skim(context: &mut AppContext, backend: &mut AppBackend) -> DiziResult {
    let widget = context.get_view_widget();

    match widget {
        WidgetType::FileBrowser => search_directory_skim(context, backend, widget)?,
        WidgetType::Playlist => search_playlist_skim(context, backend, widget)?,
        _ => {}
    }
    Ok(())
}

fn search_playlist_skim(
    context: &mut AppContext,
    backend: &mut AppBackend,
    widget: WidgetType,
) -> DiziResult {
    let options = SkimOptionsBuilder::default()
        .height("100%".to_string())
        .multi(true)
        .build()
        .unwrap();

    let items: Vec<DiziSkimItem> = context
        .server_state_ref()
        .player
        .playlist
        .list_ref()
        .iter()
        .enumerate()
        .map(|(i, item)| DiziSkimItem {
            idx: i,
            value: format!("{:03} \u{02503} {}", i + 1, item.file_name()),
        })
        .collect();

    if items.is_empty() {
        return Err(DiziError::new(
            DiziErrorKind::IoError(io::ErrorKind::InvalidData),
            "no files to select".to_string(),
        ));
    }

    let (s, r): (SkimItemSender, SkimItemReceiver) = unbounded();
    let thread = thread::spawn(move || {
        for item in items {
            let _ = s.send(sync::Arc::new(item));
        }
    });

    backend.terminal_drop();

    let skim_output = Skim::run_with(&options, Some(r));

    backend.terminal_restore()?;

    let _ = thread.join();

    if let Some(skim_output) = skim_output {
        if skim_output.final_key == Key::ESC {
            return Ok(());
        }

        let query = skim_output.query;
        if !query.is_empty() {
            context.set_search_context(SearchPattern::String(query));
        }

        for sk_item in skim_output.selected_items {
            let item: Option<&DiziSkimItem> = (*sk_item).as_any().downcast_ref::<DiziSkimItem>();

            match item {
                Some(item) => {
                    cursor_move::cursor_move_for_widget(context, widget, item.idx);
                }
                None => {
                    return Err(DiziError::new(
                        DiziErrorKind::IoError(io::ErrorKind::InvalidData),
                        "Error casting".to_string(),
                    ))
                }
            }
        }
    }

    Ok(())
}

fn search_directory_skim(
    context: &mut AppContext,
    backend: &mut AppBackend,
    widget: WidgetType,
) -> DiziResult {
    let options = SkimOptionsBuilder::default()
        .height("100%".to_string())
        .multi(true)
        .build()
        .unwrap();

    let items = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .map(|list| {
            let v: Vec<DiziSkimItem> = list
                .iter()
                .enumerate()
                .map(|(i, e)| DiziSkimItem {
                    idx: i,
                    value: e.file_name().to_string(),
                })
                .collect();
            v
        })
        .unwrap_or_default();

    if items.is_empty() {
        return Err(DiziError::new(
            DiziErrorKind::IoError(io::ErrorKind::InvalidData),
            "no files to select".to_string(),
        ));
    }

    let (s, r): (SkimItemSender, SkimItemReceiver) = unbounded();
    let thread = thread::spawn(move || {
        for item in items {
            let _ = s.send(sync::Arc::new(item));
        }
    });

    backend.terminal_drop();

    let skim_output = Skim::run_with(&options, Some(r));

    backend.terminal_restore()?;

    let _ = thread.join();

    if let Some(skim_output) = skim_output {
        if skim_output.final_key == Key::ESC {
            return Ok(());
        }

        let query = skim_output.query;
        if !query.is_empty() {
            context.set_search_context(SearchPattern::String(query));
        }

        for sk_item in skim_output.selected_items {
            let item: Option<&DiziSkimItem> = (*sk_item).as_any().downcast_ref::<DiziSkimItem>();

            match item {
                Some(item) => {
                    cursor_move::cursor_move_for_widget(context, widget, item.idx);
                }
                None => {
                    return Err(DiziError::new(
                        DiziErrorKind::IoError(io::ErrorKind::InvalidData),
                        "Error casting".to_string(),
                    ))
                }
            }
        }
    }

    Ok(())
}
