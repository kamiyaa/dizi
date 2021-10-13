use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::symbols::line::{HORIZONTAL_DOWN, HORIZONTAL_UP};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

use crate::context::AppContext;
use crate::ui::widgets::{TuiPlayer, TuiTopBar};

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

        let (default_layout, constraints): (bool, &[Constraint; 3]) =
            (true, &display_options.default_layout);

        let layout_rect = if config.display_options_ref().show_borders() {
            let area = Rect {
                y: area.top() + 1,
                height: area.height - 2,
                ..area
            };

            let block = Block::default().borders(Borders::ALL);
            let inner = block.inner(area);
            block.render(area, buf);

            let layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints.as_ref())
                .split(inner);

            // Render inner borders properly.
            {
                let top = area.top();
                let bottom = area.bottom() - 1;
                let left = layout_rect[1].left() - 1;
                let right = layout_rect[2].left();
                let intersections = Intersections {
                    top,
                    bottom,
                    left,
                    right,
                };

                intersections.render_left(buf);
                if default_layout {
                    intersections.render_right(buf);
                }
            }

            let block = Block::default().borders(Borders::RIGHT);
            let inner1 = block.inner(layout_rect[0]);
            block.render(layout_rect[0], buf);

            let block = Block::default().borders(Borders::LEFT);
            let inner3 = block.inner(layout_rect[2]);
            block.render(layout_rect[2], buf);

            vec![inner1, layout_rect[1], inner3]
        } else {
            let mut layout_rect = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(1)
                .constraints(constraints.as_ref())
                .split(area);

            layout_rect[0] = Rect {
                width: layout_rect[0].width - 1,
                ..layout_rect[0]
            };
            layout_rect[1] = Rect {
                width: layout_rect[1].width - 1,
                ..layout_rect[1]
            };
            layout_rect
        };

        TuiFolderView::new(self.context).render(layout_rect[1], buf);
        TuiPlayer::new(self.context.server_state_ref().player_ref()).render(layout_rect[2], buf);

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
