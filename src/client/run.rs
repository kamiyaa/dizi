use std::io::{BufRead, BufReader};
use std::thread;

use termion::event::Event;

use dizi_lib::error::DiziResult;

use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::AppEvent;
use crate::key_command::{AppExecute, CommandKeybind};
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

    // server listener
    {
        let stream = context.clone_stream()?;
        let event_tx = context.events.event_tx.clone();

        let _ = thread::spawn(move || {
            let cursor = BufReader::new(stream);
            for line in cursor.lines().flatten() {
                event_tx.send(AppEvent::Server(line));
            }
        });
    }

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
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    Some(CommandKeybind::CompositeKeybind(m)) => {
                        let cmd = input::get_input_while_composite(backend, context, m);

                        if let Some(command) = cmd {
                            if let Err(e) = command.execute(context, backend, &keymap_t) {
                                context.message_queue_mut().push_error(e.to_string());
                            }
                        }
                    }
                }
                preview_default::load_preview(context, backend);
                context.flush_event();
            }
            AppEvent::Server(message) => {
                input::process_server_event(context, message.as_str());
            }
            event => input::process_noninteractive(event, context),
        }
    }
    Ok(())
}
