use std::path::Path;

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

pub struct TuiTopBar<'a> {
    path: &'a Path,
}

impl<'a> TuiTopBar<'a> {
    pub fn new(path: &'a Path) -> Self {
        Self { path }
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
            Some(s) => Line::from(vec![s, Span::styled(curr_path_str, path_style)]),
            None => Line::from(vec![Span::styled(curr_path_str, path_style)]),
        };

        Paragraph::new(text).render(area, buf);
    }
}
