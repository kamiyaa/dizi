use termion::event::Event;

use dizi_commands::error::DiziResult;

use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::key_command::{AppExecute, Command, CommandKeybind};
use crate::preview::preview_default;
use crate::ui::views::TuiView;
use crate::ui::TuiBackend;
use crate::util::input;
use crate::util::to_string::ToString;

pub fn run(
    backend: &mut TuiBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> DiziResult<()> {
    context.flush_stream();

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
                if context.message_queue_ref().current_message().is_some() {
                    context.message_queue_mut().pop_front();
                }

                match keymap_t.as_ref().get(&key) {
                    None => {
                        context
                            .message_queue_mut()
                            .push_info(format!("Unmapped input: {}", key.to_string()));
                    }
                    Some(CommandKeybind::SimpleKeybind(command)) => {
                        if let Err(e) = command.execute(context, backend, &keymap_t) {
                            eprintln!("{:?}", e);
                        }
                    }
                    Some(CommandKeybind::CompositeKeybind(m)) => {}
                }
                context.flush_event();
                preview_default::load_preview(context, backend);
            }
            event => input::process_noninteractive(event, context),
        }
    }
    Ok(())
}
