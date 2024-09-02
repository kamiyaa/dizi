use std::time;

use dizi::playlist::PlaylistType;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Widget, Wrap};

use dizi::player::{PlayerState, PlayerStatus};

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

        let song = &self.player.song;
        {
            let song_name = match song {
                Some(song) => match song.music_metadata().standard_tags.get("TrackTitle") {
                    Some(title) => title.clone(),
                    None => song.file_name().to_string(),
                },
                None => " ".to_string(),
            };

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

        {
            let on_style = Style::default().fg(Color::Green);
            let off_style = Style::default().fg(Color::Black);

            let playlist_file_style = match self.player.playlist_status {
                PlaylistType::PlaylistFile => on_style,
                PlaylistType::DirectoryListing => off_style,
            };

            let playlist_directory_style = match self.player.playlist_status {
                PlaylistType::PlaylistFile => off_style,
                PlaylistType::DirectoryListing => on_style,
            };

            let player_status_style = Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD);
            let text = Line::from(vec![
                Span::styled(
                    format!("Volume: {}%  ", self.player.volume,),
                    player_status_style,
                ),
                Span::styled("[PLAYLIST] ", playlist_file_style),
                Span::styled("[DIRECTORY] ", playlist_directory_style),
            ]);

            let rect = Rect {
                y: area.y + area.height - 3,
                height: 1,
                ..area
            };
            Paragraph::new(text).render(rect, buf);
        }

        let duration_elapsed = self.player.elapsed;
        let duration_played_str = {
            let total_secs = duration_elapsed.as_secs();
            let minutes = total_secs / 60;
            let hours = total_secs / 3600;
            let seconds = total_secs % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        };
        let total_duration = song
            .as_ref()
            .and_then(|song| song.audio_metadata().total_duration)
            .unwrap_or(time::Duration::from_secs(0));
        let total_duration_str = {
            let total_secs = total_duration.as_secs();
            let minutes = total_secs / 60;
            let hours = total_secs / 3600;
            let seconds = total_secs % 60;
            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        };
        {
            let on_style = Style::default().fg(Color::Yellow);
            let off_style = Style::default().fg(Color::Black);

            let next_style = match self.player.next {
                true => on_style,
                false => off_style,
            };
            let repeat_style = match self.player.repeat {
                true => on_style,
                false => off_style,
            };
            let shuffle_style = match self.player.shuffle {
                true => on_style,
                false => off_style,
            };

            let player_status = match self.player.status {
                PlayerStatus::Playing => "\u{25B6}  ",
                PlayerStatus::Stopped => "\u{2588}\u{2588}",
                PlayerStatus::Paused => "\u{2590} \u{258C}",
            };

            let text = Line::from(vec![
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
        }

        {
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
}
