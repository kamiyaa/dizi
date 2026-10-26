use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::config::option::DisplayOption;
use crate::context::UiState;
use crate::fs::{AppDirEntry, AppDirectoryList};

pub trait DirectoryHistory {
    fn populate_to_root(
        &mut self,
        path: &Path,
        ui_state: &UiState,
        options: &DisplayOption,
    ) -> io::Result<()>;
    fn create_or_soft_update(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn create_or_reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()>;
    fn depreciate_all_entries(&mut self);

    fn depreciate_entry(&mut self, path: &Path);
}

pub type AppDirectoryHistory = HashMap<PathBuf, AppDirectoryList>;

impl DirectoryHistory for AppDirectoryHistory {
    fn populate_to_root(
        &mut self,
        path: &Path,
        ui_state: &UiState,
        options: &DisplayOption,
    ) -> io::Result<()> {
        let mut dirlists = Vec::new();

        let mut prev: Option<&Path> = None;
        for curr in path.ancestors() {
            if self.contains_key(curr) {
                let mut new_dirlist = create_dirlist_with_history(self, curr, options)?;
                if let Some(ancestor) = prev.as_ref()
                    && let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor)
                {
                    new_dirlist.set_index(Some(i), ui_state, options);
                }
                dirlists.push(new_dirlist);
            } else {
                let mut new_dirlist =
                    AppDirectoryList::from_path(curr.to_path_buf().clone(), options)?;
                if let Some(ancestor) = prev.as_ref()
                    && let Some(i) = get_index_of_value(&new_dirlist.contents, ancestor)
                {
                    new_dirlist.set_index(Some(i), ui_state, options);
                }
                dirlists.push(new_dirlist);
            }
            prev = Some(curr);
        }
        for dirlist in dirlists {
            self.insert(dirlist.file_path().to_path_buf(), dirlist);
        }
        Ok(())
    }

    fn create_or_soft_update(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let (contains_key, need_update) = if let Some(dirlist) = self.get(path) {
            (true, dirlist.need_update())
        } else {
            (false, true)
        };
        if need_update {
            let dirlist = if contains_key {
                create_dirlist_with_history(self, path, options)?
            } else {
                AppDirectoryList::from_path(path.to_path_buf(), options)?
            };
            self.insert(path.to_path_buf(), dirlist);
        }
        Ok(())
    }

    fn create_or_reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let dirlist = if self.contains_key(path) {
            create_dirlist_with_history(self, path, options)?
        } else {
            AppDirectoryList::from_path(path.to_path_buf(), options)?
        };
        self.insert(path.to_path_buf(), dirlist);
        Ok(())
    }

    fn reload(&mut self, path: &Path, options: &DisplayOption) -> io::Result<()> {
        let dirlist = create_dirlist_with_history(self, path, options)?;
        self.insert(path.to_path_buf(), dirlist);
        Ok(())
    }

    fn depreciate_all_entries(&mut self) {
        self.iter_mut().for_each(|(_, v)| v.depreciate());
    }

    fn depreciate_entry(&mut self, path: &Path) {
        if let Some(v) = self.get_mut(path) {
            v.depreciate();
        }
    }
}

fn get_index_of_value(arr: &[AppDirEntry], val: &Path) -> Option<usize> {
    arr.iter().enumerate().find_map(|(i, dir)| {
        if dir.file_path() == val {
            Some(i)
        } else {
            None
        }
    })
}

pub fn create_dirlist_with_history(
    history: &AppDirectoryHistory,
    path: &Path,
    options: &DisplayOption,
) -> io::Result<AppDirectoryList> {
    let dirlist = AppDirectoryList::from_path(path.to_path_buf(), options)?;
    Ok(merge_dirlist_with_history(history, dirlist, options))
}

/// Carries cached state from `history` over to a freshly read `dirlist`: directory sizes,
/// cursor and viewport. Does no IO, so it's safe to call on the main thread with a listing
/// read in the background.
pub fn merge_dirlist_with_history(
    history: &AppDirectoryHistory,
    mut dirlist: AppDirectoryList,
    options: &DisplayOption,
) -> AppDirectoryList {
    let path = dirlist.path.as_path();
    let contents = &mut dirlist.contents;

    // re-use directory size information on reload
    for entry in contents.iter_mut() {
        if entry.metadata.is_dir()
            && let Some(lst) = history.get(entry.file_path())
        {
            entry.metadata.update_directory_size(lst.len());
        }
    }

    // sizes may have changed, which matters when sorting by size
    let sort_options = &options.sort_options;
    contents.sort_by(|f1, f2| sort_options.compare(f1, f2));

    let former_dir_list = history.get(path);

    let contents_len = contents.len();
    let index: Option<usize> = if contents_len == 0 {
        None
    } else {
        match former_dir_list {
            Some(former) => match former.get_index() {
                Some(i) if i >= contents_len => Some(contents_len - 1),
                Some(i) => {
                    let entry = &former.contents[i];
                    contents
                        .iter()
                        .enumerate()
                        .find(|(_, e)| e.file_name() == entry.file_name())
                        .map(|(i, _)| i)
                        .or(Some(i))
                }
                None => Some(0),
            },
            None => Some(0),
        }
    };
    let viewport_index: usize = if contents_len == 0 {
        0
    } else {
        match former_dir_list {
            Some(former) => match former.first_index_for_viewport() {
                i if i >= contents_len => contents_len - 1,
                i => i,
            },
            None => 0,
        }
    };

    dirlist.index = index;
    dirlist.viewport_index = viewport_index;
    dirlist.need_update = false;
    dirlist
}

pub fn read_directory<F>(
    path: &Path,
    filter_func: F,
    options: &DisplayOption,
) -> io::Result<Vec<AppDirEntry>>
where
    F: Fn(&Result<fs::DirEntry, io::Error>) -> bool,
{
    let results: Vec<AppDirEntry> = fs::read_dir(path)?
        .filter(filter_func)
        .filter_map(|res| AppDirEntry::from(&res.ok()?, path, options).ok())
        .collect();

    Ok(results)
}
