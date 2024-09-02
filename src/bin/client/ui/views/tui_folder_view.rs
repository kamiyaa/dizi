use ratatui::buffer::Buffer;
use ratatui::layout::Rect;

use ratatui::widgets::Widget;

use crate::context::AppContext;
use crate::ui::widgets::TuiDirListDetailed;

pub struct TuiFolderView<'a> {
    pub context: &'a AppContext,
    pub focused: bool,
}

impl<'a> TuiFolderView<'a> {
    pub fn new(context: &'a AppContext, focused: bool) -> Self {
        Self { context, focused }
    }
}

impl<'a> Widget for TuiFolderView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let curr_list = self
            .context
            .tab_context_ref()
            .curr_tab_ref()
            .curr_list_ref();
        let _curr_entry = curr_list.and_then(|c| c.curr_entry_ref());

        let config = self.context.config_ref();
        let display_options = config.display_options_ref();
        let currently_playing = self.context.server_state_ref().player_ref().song.as_ref();

        // render current view
        if let Some(list) = curr_list.as_ref() {
            TuiDirListDetailed::new(list, display_options, currently_playing, self.focused)
                .render(area, buf);
            let _rect = Rect {
                x: 0,
                y: area.height - 1,
                width: area.width,
                height: 1,
            };
        }
    }
}
