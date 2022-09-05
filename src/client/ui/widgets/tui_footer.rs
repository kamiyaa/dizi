use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use dizi_lib::player::PlayerState;

pub struct TuiFooter<'a> {
    player_state: &'a PlayerState,
}

impl<'a> TuiFooter<'a> {
    pub fn new(player_state: &'a PlayerState) -> Self {
        Self { player_state }
    }
}

impl<'a> Widget for TuiFooter<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = vec![Span::styled(
            format!("Audio system: {}", self.player_state.audio_host),
            Style::default().fg(Color::Green),
        )];

        Paragraph::new(Spans::from(text)).render(area, buf);
    }
}
