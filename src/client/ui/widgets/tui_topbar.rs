use std::path::Path;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use crate::context::AppContext;

pub struct TuiTopBar<'a> {
    pub context: &'a AppContext,
    path: &'a Path,
}

impl<'a> TuiTopBar<'a> {
    pub fn new(context: &'a AppContext, path: &'a Path) -> Self {
        Self { context, path }
    }
}

impl<'a> Widget for TuiTopBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let path_style = Style::default()
            .fg(Color::LightBlue)
            .add_modifier(Modifier::BOLD);

        let mut ellipses = None;
        let mut curr_path_str = self.path.to_string_lossy().into_owned();

        if curr_path_str.len() > area.width as usize {
            if let Some(s) = self.path.file_name() {
                curr_path_str = s.to_string_lossy().into_owned();
                ellipses = Some(Span::styled("…", path_style));
            }
        }

        let text = match ellipses {
            Some(s) => Spans::from(vec![
                s,
                Span::styled(curr_path_str, path_style),
            ]),
            None => Spans::from(vec![
                Span::styled(curr_path_str, path_style),
            ]),
        };

        Paragraph::new(text).render(area, buf);
    }
}
