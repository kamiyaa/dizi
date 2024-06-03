use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget};

use dizi::player::PlayerState;

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
        let text = vec![
            Span::styled(
                format!("Audio system: {}", self.player_state.audio_host),
                Style::default().fg(Color::Green),
            ),
            Span::raw("  "),
            Span::raw(format!(
                "Channels: {}",
                self.player_state
                    .song
                    .as_ref()
                    .map(|song| song.audio_metadata())
                    .and_then(|metadata| metadata.channels)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "UNKNOWN".to_string())
            )),
            Span::raw("  "),
            Span::raw(format!(
                "Sample Rate: {} Hz",
                self.player_state
                    .song
                    .as_ref()
                    .map(|song| song.audio_metadata())
                    .and_then(|metadata| metadata.sample_rate)
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "UNKNOWN".to_string())
            )),
        ];

        Paragraph::new(Line::from(text)).render(area, buf);
    }
}
