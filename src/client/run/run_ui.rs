use std::io::{BufRead, BufReader};
use std::thread;

use termion::event::Event;
use tui::layout::{Constraint, Rect};

use dizi_lib::error::DiziResult;
use dizi_lib::request::client::ClientRequest;

use crate::config::AppKeyMapping;
use crate::context::{AppContext, QuitType};
use crate::event::process_event;
use crate::event::AppEvent;
use crate::key_command::{AppExecute, Command, CommandKeybind};
use crate::preview::preview_default;
use crate::ui::views;
use crate::ui::views::TuiView;
use crate::ui::AppBackend;
use crate::util::request::send_client_request;
use crate::util::to_string::ToString;

pub fn run_ui(
    backend: &mut AppBackend,
    context: &mut AppContext,
    keymap_t: AppKeyMapping,
) -> DiziResult<()> {
    let _ = context.flush_stream();

    // server listener
    {
        let stream = context.clone_stream()?;
        let event_tx = context.events.event_tx.clone();

        let _ = thread::spawn(move || {
            let cursor = BufReader::new(stream);
            for line in cursor.lines().flatten() {
                let _ = event_tx.send(AppEvent::Server(line));
            }
        });

        // request for server state
        let request = ClientRequest::PlayerState;
        send_client_request(context, &request)?;
    }

    while context.quit == QuitType::DoNot {
        // do the ui
        if let Ok(area) = backend.terminal_ref().size() {
            // pre-calculate some ui attributes
            calculate_ui_context(context, area);

            // render the ui
            backend.render(TuiView::new(context));
        }

        let event = match context.poll_event() {
            Ok(event) => event,
            Err(_) => return Ok(()), // TODO
        };

        match event {
            AppEvent::Termion(Event::Mouse(_event)) => {
                context.flush_event();
            }
            AppEvent::Termion(key) => {
                if context.message_queue_ref().current_message().is_some() {
                    context.message_queue_mut().pop_front();
                }
                match key {
                    // in the event where mouse input is not supported
                    // but we still want to register scroll
                    Event::Unsupported(s) if s.as_slice() == [27, 79, 65] => {
                        let command = Command::CursorMoveUp(1);
                        if let Err(e) = command.execute(context, backend, &keymap_t) {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    Event::Unsupported(s) if s.as_slice() == [27, 79, 66] => {
                        let command = Command::CursorMoveDown(1);
                        if let Err(e) = command.execute(context, backend, &keymap_t) {
                            context.message_queue_mut().push_error(e.to_string());
                        }
                    }
                    key => match keymap_t.as_ref().get(&key) {
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
                            let cmd = process_event::get_input_while_composite(backend, context, m);

                            if let Some(command) = cmd {
                                if let Err(e) = command.execute(context, backend, &keymap_t) {
                                    context.message_queue_mut().push_error(e.to_string());
                                }
                            }
                        }
                    },
                }
                preview_default::load_preview(context, backend);
                context.flush_event();
            }
            AppEvent::Server(message) => {
                if let Err(err) = process_event::process_server_event(context, message.as_str()) {
                    context.message_queue_mut().push_error(err.to_string());
                }
            }
            event => process_event::process_noninteractive(event, context),
        }
    }
    Ok(())
}

fn calculate_ui_context(context: &mut AppContext, area: Rect) {
    let area = Rect {
        y: area.top() + 1,
        height: area.height - 2,
        ..area
    };

    let column_ratio = (1, 3, 4);
    let total = (column_ratio.0 + column_ratio.1 + column_ratio.2) as u32;
    let constraints = [
        Constraint::Ratio(column_ratio.0 as u32, total),
        Constraint::Ratio(column_ratio.1 as u32, total),
        Constraint::Ratio(column_ratio.2 as u32, total),
    ];

    let layout = views::calculate_layout_with_borders(area, &constraints);
    context.ui_context_mut().layout = layout;
}
