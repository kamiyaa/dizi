use std::path::Path;
use std::time;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use dizi_lib::player::PlayerStatus;

use crate::context::Player;

pub struct TuiPlayer<'a> {
    player: &'a Player,
}

impl<'a> TuiPlayer<'a> {
    pub fn new(player: &'a Player) -> Self {
        Self { player }
    }
}

impl<'a> Widget for TuiPlayer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let player_status_style = Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);

        let player_status = match self.player.get_player_status() {
            PlayerStatus::Playing => "Playing",
            PlayerStatus::Stopped => "Stopped",
            PlayerStatus::Paused => "Paused",
        };
        let dur_played = self.player.get_duration_played();

        let dur_seconds = dur_played.as_secs();
        let dur_minutes = dur_seconds / 60;
        let dur_hours = dur_seconds / 3600;
        let duration_played_str = format!("{:02}:{:02}:{:02}", dur_hours, dur_minutes, dur_seconds);

        let song = self.player.get_song();
        let total_duration_str = match song {
            Some(song) => {
                let duration = song
                    .audio_metadata()
                    .total_duration()
                    .unwrap_or(time::Duration::from_secs(0));
                let total_secs = duration.as_secs();
                let total_mins = total_secs / 60;
                let total_hrs = total_secs / 3600;
                format!("{:02}:{:02}:{:02}", total_hrs, total_mins, total_secs)
            }
            None => {
                format!("{:02}:{:02}:{:02}", 0, 0, 0)
            }
        };

        let text = Spans::from(vec![
            Span::styled(player_status, player_status_style),
            Span::styled("\n", player_status_style),
            Span::styled(duration_played_str, player_status_style),
            Span::styled(" / ", player_status_style),
            Span::styled(total_duration_str, player_status_style),
        ]);

        Paragraph::new(text).render(area, buf);
    }
}
