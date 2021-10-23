use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::Widget;

use crate::context::AppContext;
use crate::ui::widgets::TuiDirListDetailed;

const TAB_VIEW_WIDTH: u16 = 15;

pub struct TuiFolderView<'a> {
    pub context: &'a AppContext,
    pub show_bottom_status: bool,
    pub focused: bool,
}

impl<'a> TuiFolderView<'a> {
    pub fn new(context: &'a AppContext, focused: bool) -> Self {
        Self {
            context,
            show_bottom_status: true,
            focused,
        }
    }
}

impl<'a> Widget for TuiFolderView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let curr_list = self.context.curr_list_ref();
        let curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

        // render current view
        if let Some(list) = curr_list.as_ref() {
            TuiDirListDetailed::new(list, self.focused).render(area, buf);
            let rect = Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            };
        }
    }
}
