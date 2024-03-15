mod app_theme;
mod style;

pub use self::app_theme::AppTheme;
pub use self::style::*;

const DEFAULT_CONFIG_FILE_PATH: &str = include_str!("../../../../../config/theme.toml");
