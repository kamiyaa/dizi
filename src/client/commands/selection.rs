use dizi_lib::error::DiziResult;

use crate::config::option::SelectOption;
use crate::context::AppContext;

pub fn select_files(
    context: &mut AppContext,
    pattern: &str,
    options: &SelectOption,
) -> DiziResult<()> {
    if pattern.is_empty() {
        select_without_pattern(context, options)
    } else {
        select_with_pattern(context, pattern, options)
    }
}

fn select_without_pattern(_context: &mut AppContext, _options: &SelectOption) -> DiziResult<()> {
    Ok(())
}

fn select_with_pattern(
    _context: &mut AppContext,
    _pattern: &str,
    _options: &SelectOption,
) -> DiziResult<()> {
    Ok(())
}
