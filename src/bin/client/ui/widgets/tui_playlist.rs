use std::cmp::Ordering;

use dizi::song::DiziSongEntry;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::Widget;

use unicode_width::UnicodeWidthStr;

use dizi::player::PlayerState;

use crate::util::string::UnicodeTruncate;
use crate::util::style;

const MIN_LEFT_LABEL_WIDTH: i32 = 15;

const ELLIPSIS: &str = "…";

pub struct TuiPlaylist<'a> {
    player: &'a PlayerState,
    focused: bool,
}

impl<'a> TuiPlaylist<'a> {
    pub fn new(player: &'a PlayerState, focused: bool) -> Self {
        Self { player, focused }
    }

    fn draw_playlist(&self, area: &Rect, buf: &mut Buffer) {
        let x = area.left();
        let y = area.top();

        let playlist = &self.player.playlist;

        let drawing_width = area.width as usize;
        let skip_dist = playlist.first_index_for_viewport(area.height as usize);
        let style = style::playlist_style();

        // draw every entry
        playlist
            .list_ref()
            .iter()
            .enumerate()
            .skip(skip_dist)
            .enumerate()
            .take(area.height as usize)
            .for_each(|(offset, (i, entry))| {
                print_entry(
                    buf,
                    entry,
                    i,
                    style,
                    (x + 1, y + offset as u16),
                    drawing_width - 1,
                );
            });
    }

    fn draw_selected_entry(&self, area: &Rect, buf: &mut Buffer) {
        if !self.focused {
            return;
        }

        let playlist = &self.player.playlist;
        let skip_dist = playlist.first_index_for_viewport(area.height as usize);
        let curr_index = playlist.get_cursor_index();

        if let Some(curr_index) = curr_index {
            if curr_index >= skip_dist && curr_index < skip_dist + area.height as usize {
                let song = &playlist.list_ref()[curr_index];

                let x = area.left();
                let y = area.top();

                let drawing_width = area.width as usize;
                let style = style::playlist_style().add_modifier(Modifier::REVERSED);

                // draw selected entry in a different style
                let screen_index = curr_index % area.height as usize;

                let space_fill = " ".repeat(drawing_width);
                buf.set_string(x, y + screen_index as u16, space_fill.as_str(), style);

                print_entry(
                    buf,
                    song,
                    curr_index,
                    style,
                    (x + 1, y + screen_index as u16),
                    drawing_width - 1,
                );
            }
        }
    }

    fn draw_currently_playing(&self, area: &Rect, buf: &mut Buffer) {
        let x = area.left();
        let y = area.top();

        let playlist = &self.player.playlist;
        let drawing_width = area.width as usize;
        let skip_dist = playlist.first_index_for_viewport(area.height as usize);

        // print currently playing
        if let Some(playing_index) = playlist.get_playing_index() {
            if playing_index < playlist.len()
                && playing_index >= skip_dist
                && playing_index < skip_dist + area.height as usize
            {
                let song = &playlist.list_ref()[playing_index];

                // draw selected entry in a different style
                let screen_index = playing_index % area.height as usize;

                let style = style::playing_style();

                let space_fill = " ".repeat(drawing_width);
                buf.set_string(x, y + screen_index as u16, space_fill.as_str(), style);

                print_entry(
                    buf,
                    song,
                    playing_index,
                    style,
                    (x + 1, y + screen_index as u16),
                    drawing_width - 1,
                );
            }
        }
    }
}

impl<'a> Widget for TuiPlaylist<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.width < 4 || area.height < 1 {
            return;
        }
        self.draw_playlist(&area, buf);
        self.draw_selected_entry(&area, buf);
        self.draw_currently_playing(&area, buf);
    }
}

fn print_entry(
    buf: &mut Buffer,
    entry: &DiziSongEntry,
    index: usize,
    style: Style,
    (x, y): (u16, u16),
    drawing_width: usize,
) {
    let left_label_original = format!("{:03} \u{02503} {}", index + 1, entry.file_name());
    let right_label_original = "";

    let (left_label, right_label) =
        factor_labels_for_entry(&left_label_original, right_label_original, drawing_width);

    let right_width = right_label.width();
    buf.set_stringn(x, y, left_label, drawing_width, style);
    buf.set_stringn(
        x + drawing_width as u16 - right_width as u16,
        y,
        right_label,
        drawing_width,
        style,
    );
}

fn factor_labels_for_entry<'a>(
    left_label_original: &'a str,
    right_label_original: &'a str,
    drawing_width: usize,
) -> (String, &'a str) {
    let left_label_original_width = left_label_original.width();
    let right_label_original_width = right_label_original.width();

    let left_width_remainder = drawing_width as i32 - right_label_original_width as i32;
    let width_remainder = left_width_remainder as i32 - left_label_original_width as i32;

    if drawing_width == 0 {
        ("".to_string(), "")
    } else if width_remainder >= 0 {
        (left_label_original.to_string(), right_label_original)
    } else if left_width_remainder < MIN_LEFT_LABEL_WIDTH {
        (
            if left_label_original.width() as i32 <= left_width_remainder {
                trim_file_label(left_label_original, drawing_width)
            } else {
                left_label_original.to_string()
            },
            "",
        )
    } else {
        (
            trim_file_label(left_label_original, left_width_remainder as usize),
            right_label_original,
        )
    }
}

pub fn trim_file_label(name: &str, drawing_width: usize) -> String {
    // pre-condition: string name is longer than width
    let (stem, extension) = match name.rfind('.') {
        None => (name, ""),
        Some(i) => name.split_at(i),
    };
    if drawing_width < 1 {
        "".to_string()
    } else if stem.is_empty() || extension.is_empty() {
        let full = format!("{}{}", stem, extension);
        let mut truncated = full.trunc(drawing_width - 1);
        truncated.push_str(ELLIPSIS);
        truncated
    } else {
        let ext_width = extension.width();
        match ext_width.cmp(&drawing_width) {
            Ordering::Greater => {
                // file ext does not fit
                let stem_width = drawing_width;
                let truncated_stem = stem.trunc(stem_width - 3);
                format!("{}{}.{}", truncated_stem, ELLIPSIS, ELLIPSIS)
            }
            Ordering::Equal => extension.replacen('.', ELLIPSIS, 1),
            Ordering::Less => {
                let stem_width = drawing_width - ext_width;
                let truncated_stem = stem.trunc(stem_width - 1);
                format!("{}{}{}", truncated_stem, ELLIPSIS, extension)
            }
        }
    }
}

#[cfg(test)]
mod test_factor_labels {
    use super::{factor_labels_for_entry, MIN_LEFT_LABEL_WIDTH};

    #[test]
    fn both_labels_empty_if_drawing_width_zero() {
        let left = "foo.ext";
        let right = "right";
        assert_eq!(
            ("".to_string(), ""),
            factor_labels_for_entry(left, right, 0)
        );
    }

    #[test]
    fn nothing_changes_if_all_labels_fit_easily() {
        let left = "foo.ext";
        let right = "right";
        assert_eq!(
            (left.to_string(), right),
            factor_labels_for_entry(left, right, 20)
        );
    }

    #[test]
    fn nothing_changes_if_all_labels_just_fit() {
        let left = "foo.ext";
        let right = "right";
        assert_eq!(
            (left.to_string(), right),
            factor_labels_for_entry(left, right, 12)
        );
    }

    #[test]
    fn right_label_omitted_if_left_label_would_need_to_be_shortened_below_min_left_label_width() {
        let left = "foobarbazfo.ext";
        let right = "right";
        assert!(left.chars().count() as i32 == MIN_LEFT_LABEL_WIDTH);
        assert_eq!(
            ("foobarbazfo.ext".to_string(), ""),
            factor_labels_for_entry(left, right, MIN_LEFT_LABEL_WIDTH as usize)
        );
    }

    #[test]
    fn right_label_is_kept_if_left_label_is_not_shortened_below_min_left_label_width() {
        let left = "foobarbazfoobarbaz.ext";
        let right = "right";
        assert!(left.chars().count() as i32 > MIN_LEFT_LABEL_WIDTH + right.chars().count() as i32);
        assert_eq!(
            ("foobarbazf….ext".to_string(), right),
            factor_labels_for_entry(
                left,
                right,
                MIN_LEFT_LABEL_WIDTH as usize + right.chars().count()
            )
        );
    }

    #[test]
    // regression
    fn file_name_which_is_smaller_or_equal_drawing_width_does_not_cause_right_label_to_be_omitted()
    {
        let left = "foooooobaaaaaaarbaaaaaaaaaz";
        let right = "right";
        assert!(left.chars().count() as i32 > MIN_LEFT_LABEL_WIDTH);
        assert_eq!(
            ("foooooobaaaaaaarbaaaa…".to_string(), right),
            factor_labels_for_entry(left, right, left.chars().count())
        );
    }
}

#[cfg(test)]
mod test_trim_file_label {
    use super::trim_file_label;

    #[test]
    fn dotfiles_get_an_ellipsis_at_the_end_if_they_dont_fit() {
        let label = ".joshuto";
        assert_eq!(".jos…".to_string(), trim_file_label(label, 5));
    }

    #[test]
    fn dotless_files_get_an_ellipsis_at_the_end_if_they_dont_fit() {
        let label = "Desktop";
        assert_eq!("Desk…".to_string(), trim_file_label(label, 5));
    }

    #[test]
    fn if_the_extension_doesnt_fit_show_stem_with_double_ellipse() {
        let label = "12345678.12345678910";
        assert_eq!("12345….…".to_string(), trim_file_label(label, 8));
    }

    #[test]
    fn if_just_the_extension_fits_its_shown_with_an_ellipsis_instead_of_a_dot() {
        let left = "foo.ext";
        assert_eq!("…ext".to_string(), trim_file_label(left, 4));
    }

    #[test]
    fn if_the_extension_fits_the_stem_is_truncated_with_an_appended_ellipsis_1() {
        let left = "foo.ext";
        assert_eq!("….ext".to_string(), trim_file_label(left, 5));
    }

    #[test]
    fn if_the_extension_fits_the_stem_is_truncated_with_an_appended_ellipsis_2() {
        let left = "foo.ext";
        assert_eq!("f….ext".to_string(), trim_file_label(left, 6));
    }

    #[test]
    fn if_the_name_is_truncated_after_a_full_width_character_the_ellipsis_is_shown_correctly() {
        let left = "🌕🌕🌕";
        assert_eq!("🌕…".to_string(), trim_file_label(left, 4));
    }

    #[test]
    fn if_the_name_is_truncated_within_a_full_width_character_the_ellipsis_is_shown_correctly() {
        let left = "🌕🌕🌕";
        assert_eq!("🌕🌕…".to_string(), trim_file_label(left, 5));
    }
}
