use globset::Glob;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::config::option::SelectOption;
use crate::context::AppContext;

use super::cursor_move;

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

fn select_without_pattern(context: &mut AppContext, options: &SelectOption) -> DiziResult<()> {
    Ok(())
}

fn select_with_pattern(
    context: &mut AppContext,
    pattern: &str,
    options: &SelectOption,
) -> DiziResult<()> {
    Ok(())
}
