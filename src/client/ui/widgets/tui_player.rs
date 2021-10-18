use std::path::Path;
use std::time;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use dizi_lib::player::{PlayerState, PlayerStatus};

pub struct TuiPlayer<'a> {
    player: &'a PlayerState,
}

impl<'a> TuiPlayer<'a> {
    pub fn new(player: &'a PlayerState) -> Self {
        Self { player }
    }
}

impl<'a> Widget for TuiPlayer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height <= 8 {
            return;
        }

        let player_status_style = Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);

        let player_status = match self.player.get_player_status() {
            PlayerStatus::Playing => "Playing",
            PlayerStatus::Stopped => "Stopped",
            PlayerStatus::Paused => "Paused",
        };
        let duration_played_str = {
            let duration = self.player.get_elapsed();

            let total_secs = duration.as_secs();
            let minutes = total_secs / 60;
            let hours = total_secs / 3600;
            let seconds = total_secs - hours * 3600 - minutes * 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        };

        let song = self.player.get_song();
        let total_duration_str = match song {
            Some(song) => {
                let duration = song
                    .audio_metadata()
                    .total_duration()
                    .unwrap_or(time::Duration::from_secs(0));
                let total_secs = duration.as_secs();
                let minutes = total_secs / 60;
                let hours = total_secs / 3600;
                let seconds = total_secs - hours * 3600 - minutes * 60;
                format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
            }
            None => {
                format!("{:02}:{:02}:{:02}", 0, 0, 0)
            }
        };

        let song_name = match song {
            Some(song) => song
                .file_path()
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            None => "".to_string(),
        };

        buf.set_string(
            area.x,
            area.y,
            format!("{} [{}]", player_status, song_name),
            player_status_style,
        );
        buf.set_string(
            area.x,
            area.y + 2,
            format!("{} / {}", duration_played_str, total_duration_str),
            Style::default(),
        );
        buf.set_string(
            area.x,
            area.y + 3,
            format!("Volume: {}%", self.player.get_volume()),
            player_status_style,
        );

        let on_style = Style::default().fg(Color::Yellow);
        let off_style = Style::default().fg(Color::Black);

        let next_style = if self.player.next_enabled() {
            on_style
        } else {
            off_style
        };

        let repeat_style = if self.player.repeat_enabled() {
            on_style
        } else {
            off_style
        };

        let shuffle_style = if self.player.shuffle_enabled() {
            on_style
        } else {
            off_style
        };

        let text = Spans::from(vec![
            Span::styled("[NEXT] ", next_style),
            Span::styled("[REPEAT] ", repeat_style),
            Span::styled("[SHUFFLE] ", shuffle_style),
        ]);

        let rect = Rect {
            y: area.y + 4,
            height: area.height - 4,
            ..area
        };
        Paragraph::new(text).render(rect, buf);
    }
}
