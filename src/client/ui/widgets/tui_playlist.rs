use std::path::Path;
use std::time;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, Widget};

use dizi_lib::player::{PlayerState, PlayerStatus};

pub struct TuiPlaylist<'a> {
    player: &'a PlayerState,
}

impl<'a> TuiPlaylist<'a> {
    pub fn new(player: &'a PlayerState) -> Self {
        Self { player }
    }
}

impl<'a> Widget for TuiPlaylist<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height <= 8 {
            return;
        }
    }
}
