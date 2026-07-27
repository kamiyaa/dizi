use ratatui::style::Style;

use crate::fs::{FileType, JoshutoDirEntry, LinkType};
use crate::utils::unix;

use crate::THEME_T;

pub fn playing_style() -> Style {
    Style::default()
        .fg(THEME_T.playing.fg)
        .bg(THEME_T.playing.bg)
        .add_modifier(THEME_T.playing.modifier)
}

pub fn playlist_style() -> Style {
    Style::default()
        .fg(THEME_T.playlist.fg)
        .bg(THEME_T.playlist.bg)
        .add_modifier(THEME_T.playlist.modifier)
}

pub fn entry_style(entry: &JoshutoDirEntry) -> Style {
    let metadata = &entry.metadata;
    let filetype = metadata.file_type();
    let linktype = metadata.link_type();

    default_style(entry, linktype, filetype)
}

fn default_style(entry: &JoshutoDirEntry, linktype: &LinkType, filetype: FileType) -> Style {
    match linktype {
        LinkType::Symlink { valid: true, .. } => Style::default()
            .fg(THEME_T.link.fg)
            .bg(THEME_T.link.bg)
            .add_modifier(THEME_T.link.modifier),
        LinkType::Symlink { valid: false, .. } => Style::default()
            .fg(THEME_T.link_invalid.fg)
            .bg(THEME_T.link_invalid.bg)
            .add_modifier(THEME_T.link_invalid.modifier),
        LinkType::Normal => match filetype {
            FileType::Directory => Style::default()
                .fg(THEME_T.directory.fg)
                .bg(THEME_T.directory.bg)
                .add_modifier(THEME_T.directory.modifier),
            FileType::File => file_style(entry),
            FileType::Link => Style::default()
                .fg(THEME_T.link.fg)
                .bg(THEME_T.link.bg)
                .add_modifier(THEME_T.link.modifier),
            _ => Style::default()
                .fg(THEME_T.socket.fg)
                .bg(THEME_T.socket.bg)
                .add_modifier(THEME_T.socket.modifier),
        },
    }
}

fn file_style(entry: &JoshutoDirEntry) -> Style {
    let metadata = &entry.metadata;
    if unix::is_executable(metadata.mode) {
        Style::default()
            .fg(THEME_T.executable.fg)
            .bg(THEME_T.executable.bg)
            .add_modifier(THEME_T.executable.modifier)
    } else {
        match entry.file_path().extension() {
            None => Style::default(),
            Some(os_str) => match os_str.to_str() {
                None => Style::default(),
                Some(s) => match THEME_T.ext.get(s) {
                    None => Style::default(),
                    Some(t) => Style::default().fg(t.fg).bg(t.bg).add_modifier(t.modifier),
                },
            },
        }
    }
}
