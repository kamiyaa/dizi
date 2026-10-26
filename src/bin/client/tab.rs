use std::collections::HashMap;
use std::path;

use crate::config::option::DisplayOption;
use crate::context::UiState;
use crate::fs::AppDirectoryList;
use crate::fs::{AppDirectoryHistory, DirectoryHistory};
use crate::preview::PreviewDirState;

type HistoryMetadata = HashMap<path::PathBuf, PreviewDirState>;

#[derive(Debug)]
pub struct JoshutoTab {
    pub cwd: path::PathBuf,
    // history is just a HashMap, so we have this property to store last workdir
    pub previous_dir: Option<path::PathBuf>,
    pub history: AppDirectoryHistory,
    pub history_metadata: HistoryMetadata,
}

impl JoshutoTab {
    pub fn new(
        cwd: path::PathBuf,
        ui_state: &UiState,
        options: &DisplayOption,
    ) -> std::io::Result<Self> {
        let mut history = AppDirectoryHistory::new();
        history.populate_to_root(cwd.as_path(), ui_state, options)?;

        Ok(Self {
            cwd,
            previous_dir: None,
            history,
            history_metadata: HashMap::new(),
        })
    }

    pub fn get_cwd(&self) -> &path::Path {
        self.cwd.as_path()
    }

    pub fn set_cwd(&mut self, cwd: &path::Path) {
        self.previous_dir = Some(self.cwd.to_path_buf());
        self.cwd = cwd.to_path_buf();
    }

    pub fn previous_dir(&self) -> Option<&path::Path> {
        self.previous_dir.as_deref()
    }

    pub fn curr_list_ref(&self) -> Option<&AppDirectoryList> {
        self.history.get(self.get_cwd())
    }

    pub fn parent_list_ref(&self) -> Option<&AppDirectoryList> {
        let parent = self.get_cwd().parent()?;
        self.history.get(parent)
    }

    pub fn child_list_ref(&self) -> Option<&AppDirectoryList> {
        let curr_list = self.curr_list_ref()?;
        let index = curr_list.get_index()?;
        let path = curr_list.contents[index].file_path();
        self.history.get(path)
    }

    pub fn curr_list_mut(&mut self) -> Option<&mut AppDirectoryList> {
        self.history.get_mut(self.cwd.as_path())
    }

    pub fn parent_list_mut(&mut self) -> Option<&mut AppDirectoryList> {
        let parent = self.cwd.parent()?;
        self.history.get_mut(parent)
    }

    #[allow(dead_code)]
    pub fn child_list_mut(&mut self) -> Option<&mut AppDirectoryList> {
        let child_path = {
            let curr_list = self.curr_list_ref()?;
            let index = curr_list.get_index()?;
            curr_list.contents[index].file_path().to_path_buf()
        };

        self.history.get_mut(child_path.as_path())
    }
}
