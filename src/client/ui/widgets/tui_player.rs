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
        if area.height <= 4 {
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
            let duration = self.player.get_duration_played();

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

        buf.set_string(area.x, area.y, player_status, player_status_style);
        buf.set_string(area.x, area.y + 1, duration_played_str, player_status_style);
        buf.set_string(area.x, area.y + 2, total_duration_str, player_status_style);
        buf.set_string(
            area.x,
            area.y + 3,
            format!("Volume: {}%", self.player.get_volume()),
            player_status_style,
        );
    }
}
