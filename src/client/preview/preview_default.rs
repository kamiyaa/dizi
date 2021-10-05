use std::path;

use crate::context::AppContext;
use crate::fs::Metadata;
use crate::preview::preview_dir;
use crate::ui::TuiBackend;

pub fn load_preview_path(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    p: path::PathBuf,
    metadata: Metadata,
) {
    if metadata.is_dir() {
        let need_to_load = context
            .history_ref()
            .get(p.as_path())
            .map(|e| e.need_update())
            .unwrap_or(true);

        if need_to_load {
            preview_dir::Background::load_preview(context, p);
        }
    }
}

pub fn load_preview(context: &mut AppContext, backend: &mut TuiBackend) {
    let mut load_list = Vec::with_capacity(2);

    match context.curr_list_ref() {
        Some(curr_list) => {
            if let Some(index) = curr_list.index {
                let entry = &curr_list.contents[index];
                load_list.push((entry.file_path().to_path_buf(), entry.metadata.clone()));
            }
        }
        None => {
            if let Ok(metadata) = Metadata::from(context.cwd()) {
                load_list.push((context.cwd().to_path_buf(), metadata));
            }
        }
    }

    for (path, metadata) in load_list {
        load_preview_path(context, backend, path, metadata);
    }
}
