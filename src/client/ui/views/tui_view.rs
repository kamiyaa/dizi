use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::symbols::line::{HORIZONTAL_DOWN, HORIZONTAL_UP};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::config::option::{LayoutComposition, WidgetType};
use crate::context::AppContext;
use crate::ui::widgets::{TuiPlayer, TuiPlaylist, TuiTopBar};

use crate::LAYOUT_T;

use super::TuiFolderView;

pub struct TuiView<'a> {
    pub context: &'a AppContext,
    pub show_bottom_status: bool,
}

impl<'a> TuiView<'a> {
    pub fn new(context: &'a AppContext) -> Self {
        Self {
            context,
            show_bottom_status: true,
        }
    }
}

impl<'a> Widget for TuiView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let default_layout = [Constraint::Ratio(1, 1)];
        let layout_rect = Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(1)
            .constraints(default_layout)
            .split(area);

        render_widget(self.context, &LAYOUT_T.layout, layout_rect[0], buf);

        if let Some(msg) = self.context.message_queue_ref().current_message() {
            let rect = Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            };

            let text = Span::styled(msg.content.as_str(), msg.style);
            Paragraph::new(text)
                .wrap(Wrap { trim: true })
                .render(rect, buf);
        }

        let topbar_width = area.width;
        let rect = Rect {
            x: 0,
            y: 0,
            width: topbar_width,
            height: 1,
        };
        TuiTopBar::new(self.context, self.context.cwd()).render(rect, buf);
    }
}

pub fn render_widget(
    context: &AppContext,
    layout: &LayoutComposition,
    area: Rect,
    buf: &mut Buffer,
) {
    let focused_panel_style = Style::default().fg(Color::Blue);
    let unfocused_panel_style = Style::default();

    let current_view_widget = context.get_view_widget();

    match layout {
        LayoutComposition::Simple {
            widget,
            ratio: _,
            border,
            title: _,
        } => {
            let focused = current_view_widget == *widget;

            let border_style = if focused {
                focused_panel_style
            } else {
                unfocused_panel_style
            };

            let rect = if *border {
                let block = Block::default()
                    .border_style(border_style)
                    .borders(Borders::ALL);
                let inner = block.inner(area);
                block.render(area, buf);
                inner
            } else {
                area
            };
            match widget {
                WidgetType::FileBrowser => TuiFolderView::new(context, focused).render(rect, buf),
                WidgetType::MusicPlayer => {
                    TuiPlayer::new(context.server_state_ref().player_ref()).render(rect, buf)
                }
                WidgetType::Playlist => {
                    TuiPlaylist::new(context.server_state_ref().player_ref(), focused)
                        .render(rect, buf)
                }
            }
        }
        LayoutComposition::Composite {
            direction,
            widgets,
            ratio: _,
        } => {
            let widget_sizes: Vec<usize> = widgets.iter().map(|w| w.ratio()).collect();
            let widget_size_sum = widget_sizes.iter().map(|n| *n as u32).sum();
            let constraints: Vec<Constraint> = widget_sizes
                .iter()
                .map(|n| Constraint::Ratio(*n as u32, widget_size_sum))
                .collect();

            let layout_rect = Layout::default()
                .direction(direction.clone())
                .constraints(constraints)
                .split(area);
            for (widget, rect) in widgets.iter().zip(layout_rect) {
                render_widget(context, widget, rect, buf);
            }
        }
    }
}

pub fn calculate_layout_with_borders(area: Rect, constraints: &[Constraint]) -> Vec<Rect> {
    let block = Block::default().borders(Borders::ALL);
    let inner = block.inner(area);

    let layout_rect = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints.as_ref())
        .split(inner);

    let block = Block::default().borders(Borders::RIGHT);
    let inner1 = block.inner(layout_rect[0]);

    let block = Block::default().borders(Borders::LEFT);
    let inner3 = block.inner(layout_rect[2]);

    vec![inner1, layout_rect[1], inner3]
}
