use termion::event::Event;

use dizi_commands::error::DiziResult;

use crate::key_command::{AppExecute, Command, CommandKeybind};
use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::preview::preview_default;
use crate::ui::views::TuiView;
use crate::ui::TuiBackend;
use crate::util::input;

pub fn run(
    backend: &mut TuiBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> DiziResult<()> {

    while context.quit == QuitType::DoNot {
        backend.render(TuiView::new(&context));

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            AppEvent::Termion(Event::Mouse(event)) => {
                context.flush_event();
            }
            AppEvent::Termion(key) => {
                match keymap_t.as_ref().get(&key) {
                    None => {
                        // handle error
                    }
                    Some(CommandKeybind::SimpleKeybind(command)) => {
                        if let Err(e) = command.execute(context, backend, &keymap_t) {
                            // handle error
                        }
                    }
                    Some(CommandKeybind::CompositeKeybind(m)) => {
                    }
                }
                context.flush_event();
                preview_default::load_preview(context, backend);
            }
            event => input::process_noninteractive(event, context),
        }
    }
    Ok(())
}
