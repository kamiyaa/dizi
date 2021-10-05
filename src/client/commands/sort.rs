use dizi_commands::error::DiziResult;

use crate::context::AppContext;
use crate::history::DirectoryHistory;
use crate::util::sort_type::SortType;

use super::reload;

pub fn set_sort(context: &mut AppContext, method: SortType) -> DiziResult<()> {
    context
        .config_mut()
        .sort_options_mut()
        .set_sort_method(method);
    context.history_mut().depreciate_all_entries();
    refresh(context)
}

pub fn toggle_reverse(context: &mut AppContext) -> DiziResult<()> {
    let reversed = !context.config_ref().sort_options_ref().reverse;
    context.config_mut().sort_options_mut().reverse = reversed;

    context.history_mut().depreciate_all_entries();
    refresh(context)
}

fn refresh(context: &mut AppContext) -> DiziResult<()> {
    reload::soft_reload(context)?;
    Ok(())
}
