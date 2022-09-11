use std::time;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget, Wrap};

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
        if area.height < 5 {
            return;
        }

        let player_status_style = Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD);

        let player_status = match self.player.get_player_status() {
            PlayerStatus::Playing => ">>",
            PlayerStatus::Stopped => "\u{2588}\u{2588}",
            PlayerStatus::Paused => "||",
        };

        let duration_elapsed = self.player.get_elapsed();

        let song = self.player.get_song();
        let song_name = match song {
            Some(song) => match song.music_metadata().standard_tags.get("TrackTitle") {
                Some(title) => title.clone(),
                None => song.file_name().to_string(),
            },
            None => " ".to_string(),
        };

        {
            let p_rect = Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 4,
            };
            Paragraph::new(song_name)
                .style(Style::default())
                .wrap(Wrap { trim: true })
                .render(p_rect, buf);
        }
        buf.set_string(
            area.x,
            area.y + area.height - 3,
            format!("Volume: {}%", self.player.get_volume()),
            player_status_style,
        );

        let duration_played_str = {
            let total_secs = duration_elapsed.as_secs();
            let minutes = total_secs / 60;
            let hours = total_secs / 3600;
            let seconds = total_secs % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        };
        let total_duration = song
            .and_then(|song| song.audio_metadata().total_duration)
            .unwrap_or(time::Duration::from_secs(0));
        let total_duration_str = {
            let total_secs = total_duration.as_secs();
            let minutes = total_secs / 60;
            let hours = total_secs / 3600;
            let seconds = total_secs % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        };

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
            Span::raw(format!(
                "{} {} / {}   ",
                player_status, duration_played_str, total_duration_str
            )),
            Span::styled("[NEXT] ", next_style),
            Span::styled("[REPEAT] ", repeat_style),
            Span::styled("[SHUFFLE] ", shuffle_style),
        ]);

        let rect = Rect {
            y: area.y + area.height - 2,
            height: 1,
            ..area
        };
        Paragraph::new(text).render(rect, buf);

        let total_duration = total_duration.as_secs();
        if total_duration > 0 {
            let secs = duration_elapsed.as_secs();
            // draw a progress bar
            let progress_bar_width =
                (secs as f32 / total_duration as f32 * area.width as f32) as usize;

            let progress_bar_space = " ".repeat(progress_bar_width);
            let style = Style::default().bg(Color::Blue);
            buf.set_stringn(
                area.x,
                area.y + area.height - 1,
                progress_bar_space,
                area.width as usize,
                style,
            );
        }
    }
}
