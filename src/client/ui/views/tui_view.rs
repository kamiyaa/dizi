use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::symbols::line::{HORIZONTAL_DOWN, HORIZONTAL_UP};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::context::AppContext;
use crate::ui::widgets::{TuiPlayer, TuiPlaylist, TuiTopBar};

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
        let config = self.context.config_ref();
        let display_options = config.display_options_ref();

        let default_layout = [Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)];

        let mut layout_rect = Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(1)
            .constraints(default_layout)
            .split(area);

        let focused_panel_style = Style::default().fg(Color::Blue);

        let block = Block::default()
            .border_style(focused_panel_style)
            .borders(Borders::ALL);
        let border_inner_rect = block.inner(layout_rect[0]);
        block.render(layout_rect[0], buf);
        layout_rect[0] = border_inner_rect;

        let nested_layout = [Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)];
        let mut layout_rect2 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(nested_layout)
            .split(layout_rect[1]);

        let block = Block::default().borders(Borders::ALL);
        let border_inner_rect = block.inner(layout_rect2[0]);
        block.render(layout_rect2[0], buf);
        layout_rect2[0] = border_inner_rect;

        let block = Block::default().borders(Borders::ALL);
        let border_inner_rect = block.inner(layout_rect2[1]);
        block.render(layout_rect2[1], buf);
        layout_rect2[1] = border_inner_rect;

        TuiFolderView::new(self.context).render(layout_rect[0], buf);
        TuiPlayer::new(self.context.server_state_ref().player_state_ref())
            .render(layout_rect2[0], buf);
        TuiPlaylist::new(self.context.server_state_ref().player_state_ref())
            .render(layout_rect2[1], buf);

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

struct Intersections {
    top: u16,
    bottom: u16,
    left: u16,
    right: u16,
}

impl Intersections {
    fn render_left(&self, buf: &mut Buffer) {
        buf.get_mut(self.left, self.top).set_symbol(HORIZONTAL_DOWN);
        buf.get_mut(self.left, self.bottom)
            .set_symbol(HORIZONTAL_UP);
    }
    fn render_right(&self, buf: &mut Buffer) {
        buf.get_mut(self.right, self.top)
            .set_symbol(HORIZONTAL_DOWN);
        buf.get_mut(self.right, self.bottom)
            .set_symbol(HORIZONTAL_UP);
    }
}
