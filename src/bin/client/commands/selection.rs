use dizi::error::AppResult;

use crate::config::option::SelectOption;
use crate::context::AppState;

pub fn select_files(context: &mut AppState, pattern: &str, options: &SelectOption) -> AppResult {
    if pattern.is_empty() {
        select_without_pattern(context, options)
    } else {
        select_with_pattern(context, pattern, options)
    }
}

fn select_without_pattern(_context: &mut AppState, _options: &SelectOption) -> AppResult {
    Ok(())
}

fn select_with_pattern(
    _context: &mut AppState,
    _pattern: &str,
    _options: &SelectOption,
) -> AppResult {
    Ok(())
}
