use serde_derive::Deserialize;
use serde_json::{Number, Value};

use tui::layout::Direction;

use crate::config::Flattenable;
use crate::util::display_option::{default_column_ratio, DisplayOption};

use super::SortRawOption;

const fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize)]
pub type RawAppLayout = serde_json::Map;;

impl Flattenable<AppLayout> for RawAppLayout {
    fn flatten(self) -> AppLayout {

    }
}

macro_rules! json_obj_stub {
    ($type_name:ident, $ratio:ident) => {
        let mut table = serde_json::Map::new();
        table.insert("type".to_string(),
            Value::String($type_name.to_string()));
        table.insert("ratio".to_string(),
            Value::Number(Number::from_f64($ratio as f64)));
        table
    }
}

macro_rules! json_obj_composite {
    ($type_name:ident) => {
        let mut table = serde_json::Map::new();
        table.insert("type".to_string(),
            Value::String($type_name.to_string()));
        table
    }
}

impl std::default::Default for RawAppLayout {
    fn default() -> Self {
        let file_browser = json_obj_stub!("file_browser", 1);
        let music_player = json_obj_stub!("music_player", 1);
        let playlist = json_obj_stub!("playlist", 1);

        let mut composite1 = json_obj_composite!("composite");
        composite1.insert("direction".to_string(),
            Value::String("vertical".to_string()));
        composite1.insert("panels".to_string(),
            Value::Array(vec![Value::Object(music_player),
                Value::Object(playlist)]));

        let mut root = json_obj_composite!("composite");
        composite1.insert("direction".to_string(),
            Value::String("horizontal".to_string()));
        composite1.insert("panels".to_string(),
            Value::Array(vec![
                Value::Object(file_browser),
                Value::Object(composite1)]));
        root
    }
}

#[derive(Clone, Copy, Debug)]
pub enum PanelType {
    FileBrowser,
    MusicPlayer,
    Playlist,
}

#[derive(Clone, Debug)]
pub enum LayoutPanel {
    Singular(SingularLayout)
    Composite(CompositeLayout),
}

#[derive(Clone, Debug)]
pub struct SingularLayout {
    pub _type: PanelType,
    pub ratio: usize,
}

#[derive(Clone, Debug)]
pub struct CompositeLayout {
    pub direction: layout::Direction,
    pub panels: Vec<LayoutPanel>,
}
