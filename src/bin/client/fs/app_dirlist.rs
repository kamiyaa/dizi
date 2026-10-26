use std::cmp;
use std::path;
use std::slice::Iter;

use crate::config::option::DisplayOption;
use crate::context::UiState;
use crate::fs::read_directory;
use crate::fs::{AppDirEntry, AppFileMetadata};

#[derive(Clone, Debug)]
pub struct AppDirectoryList {
    pub path: path::PathBuf,
    pub contents: Vec<AppDirEntry>,
    pub metadata: AppFileMetadata,
    /// The cursor position in this dir list
    pub index: Option<usize>,
    /// The index in this dir list to start with when rendering the list
    pub viewport_index: usize,
    pub need_update: bool,
}

impl AppDirectoryList {
    pub fn from_path(path: path::PathBuf, options: &DisplayOption) -> std::io::Result<Self> {
        let filter_func = options.filter_func();
        let mut contents = read_directory(path.as_path(), filter_func, options)?;

        let sort_options = &options.sort_options;
        contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

        let index = if contents.is_empty() { None } else { Some(0) };

        let metadata = AppFileMetadata::from(&path)?;

        Ok(Self {
            path,
            contents,
            metadata,
            need_update: false,
            index,
            viewport_index: index.unwrap_or_default(),
        })
    }

    pub fn get_index(&self) -> Option<usize> {
        self.index
    }

    fn update_viewport(&mut self, ui_state: &UiState, options: &DisplayOption) {
        if let Some(ix) = self.index {
            let height = ui_state.layout[0].height as usize;

            // get scroll buffer size, corrected in case of too small terminal
            let scroll_offset = if height < 4 {
                0
            } else if options.scroll_offset * 2 > height - 1 {
                height / 2 - 1
            } else {
                options.scroll_offset
            };

            // calculate viewport
            let viewport_end = self.viewport_index + height;
            if (viewport_end as i16 - ix as i16 - 1) < scroll_offset as i16 {
                // cursor too low
                self.viewport_index = ix + scroll_offset - height + 1;
            } else if (ix as i16 - self.viewport_index as i16) < scroll_offset as i16 {
                // cursor too high
                self.viewport_index = cmp::max(ix as i16 - scroll_offset as i16, 0) as usize;
            }
        } else {
            self.viewport_index = 0;
        }
    }

    pub fn set_index(&mut self, index: Option<usize>, ui_state: &UiState, options: &DisplayOption) {
        if index == self.index {
            return;
        }
        self.index = index;
        if !ui_state.layout.is_empty() {
            self.update_viewport(ui_state, options);
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, AppDirEntry> {
        self.contents.iter()
    }

    pub fn len(&self) -> usize {
        self.contents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    pub fn modified(&self) -> bool {
        // follow symlinks, like `AppFileMetadata::from` does for `self.metadata`
        let metadata = std::fs::metadata(self.file_path());
        metadata
            .and_then(|m| m.modified())
            .map(|m| m > self.metadata.modified())
            .unwrap_or(false)
    }

    pub fn depreciate(&mut self) {
        self.need_update = true;
    }

    pub fn need_update(&self) -> bool {
        self.need_update || self.modified()
    }

    pub fn file_path(&self) -> &path::PathBuf {
        &self.path
    }

    pub fn curr_entry_ref(&self) -> Option<&AppDirEntry> {
        self.get_curr_ref_(self.index?)
    }

    /// Returns the index of the first entry to be printed in a UI dir list
    pub fn first_index_for_viewport(&self) -> usize {
        self.viewport_index
    }

    fn get_curr_ref_(&self, index: usize) -> Option<&AppDirEntry> {
        if index < self.contents.len() {
            Some(&self.contents[index])
        } else {
            None
        }
    }
}
