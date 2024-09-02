use serde::Deserialize;

use crate::config::option::{SortOption, SortType, SortTypes};

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub struct SortOptionRaw {
    #[serde(default = "default_true")]
    pub directories_first: bool,
    #[serde(default)]
    pub case_sensitive: bool,
    #[serde(default = "default_true")]
    pub reverse: bool,
    #[serde(default)]
    pub sort_method: Option<String>,
}

impl From<SortOptionRaw> for SortOption {
    fn from(raw: SortOptionRaw) -> Self {
        let sort_method = match raw.sort_method.as_ref() {
            Some(s) => SortType::parse(s).unwrap_or(SortType::Natural),
            None => SortType::Natural,
        };

        let mut sort_methods = SortTypes::default();
        sort_methods.reorganize(sort_method);

        Self {
            directories_first: raw.directories_first,
            case_sensitive: raw.case_sensitive,
            reverse: raw.reverse,
            sort_methods,
        }
    }
}

impl std::default::Default for SortOptionRaw {
    fn default() -> Self {
        Self {
            directories_first: true,
            case_sensitive: false,
            reverse: true,
            sort_method: None,
        }
    }
}
