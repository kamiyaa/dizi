use std::path::Path;

use dizi_lib::error::DiziResult;

use crate::context::AppContext;
use crate::events::AppEvent;

pub fn quit_server(context: &mut AppContext) {
    context.event_req().send(AppEvent::Quit);
}
