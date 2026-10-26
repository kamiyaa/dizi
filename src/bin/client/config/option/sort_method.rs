use std::cmp;
use std::collections::VecDeque;

use serde::Deserialize;

use crate::config::option::FileSortConfig;
use crate::fs::AppDirEntry;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum SortMethod {
    Lexical,
    Mtime,
    Natural,
    Size,
    Ext,
}

impl SortMethod {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "lexical" => Some(SortMethod::Lexical),
            "mtime" => Some(SortMethod::Mtime),
            "natural" => Some(SortMethod::Natural),
            "size" => Some(SortMethod::Size),
            "ext" => Some(SortMethod::Ext),
            _ => None,
        }
    }
    pub const fn as_str(&self) -> &str {
        match *self {
            SortMethod::Lexical => "lexical",
            SortMethod::Mtime => "mtime",
            SortMethod::Natural => "natural",
            SortMethod::Size => "size",
            SortMethod::Ext => "ext",
        }
    }
    pub fn cmp(
        &self,
        f1: &AppDirEntry,
        f2: &AppDirEntry,
        sort_option: &FileSortConfig,
    ) -> cmp::Ordering {
        match &self {
            SortMethod::Natural => natural_sort(f1, f2, sort_option),
            SortMethod::Lexical => lexical_sort(f1, f2, sort_option),
            SortMethod::Size => size_sort(f1, f2),
            SortMethod::Mtime => mtime_sort(f1, f2),
            SortMethod::Ext => ext_sort(f1, f2),
        }
    }
}

impl std::fmt::Display for SortMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct SortMethodList {
    pub list: VecDeque<SortMethod>,
}

impl SortMethodList {
    pub fn reorganize(&mut self, st: SortMethod) {
        self.list.push_front(st);
        self.list.pop_back();
    }

    pub fn cmp(
        &self,
        f1: &AppDirEntry,
        f2: &AppDirEntry,
        sort_option: &FileSortConfig,
    ) -> cmp::Ordering {
        for st in &self.list {
            let res = st.cmp(f1, f2, sort_option);
            if res != cmp::Ordering::Equal {
                return res;
            }
        }
        cmp::Ordering::Equal
    }
}

impl std::default::Default for SortMethodList {
    fn default() -> Self {
        let list: VecDeque<SortMethod> = vec![
            SortMethod::Natural,
            SortMethod::Lexical,
            SortMethod::Size,
            SortMethod::Ext,
            SortMethod::Mtime,
        ]
        .into_iter()
        .collect();

        Self { list }
    }
}

fn mtime_sort(file1: &AppDirEntry, file2: &AppDirEntry) -> cmp::Ordering {
    file1.metadata.modified().cmp(&file2.metadata.modified())
}

fn size_sort(file1: &AppDirEntry, file2: &AppDirEntry) -> cmp::Ordering {
    file1.metadata.len().cmp(&file2.metadata.len())
}

fn ext_sort(file1: &AppDirEntry, file2: &AppDirEntry) -> cmp::Ordering {
    let f1_ext = file1.ext().unwrap_or_default();
    let f2_ext = file2.ext().unwrap_or_default();
    alphanumeric_sort::compare_str(f1_ext, f2_ext)
}

fn lexical_sort(f1: &AppDirEntry, f2: &AppDirEntry, sort_option: &FileSortConfig) -> cmp::Ordering {
    let f1_name = f1.file_name();
    let f2_name = f2.file_name();
    if sort_option.case_sensitive {
        f1_name.cmp(f2_name)
    } else {
        let f1_name = f1_name.to_lowercase();
        let f2_name = f2_name.to_lowercase();
        f1_name.cmp(&f2_name)
    }
}

fn natural_sort(f1: &AppDirEntry, f2: &AppDirEntry, sort_option: &FileSortConfig) -> cmp::Ordering {
    let f1_name = f1.file_name();
    let f2_name = f2.file_name();
    if sort_option.case_sensitive {
        alphanumeric_sort::compare_str(f1_name, f2_name)
    } else {
        let f1_name = f1_name.to_lowercase();
        let f2_name = f2_name.to_lowercase();
        alphanumeric_sort::compare_str(f1_name, f2_name)
    }
}
