use std::str::FromStr;

use tui::layout::Direction;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::config::LayoutCompositionCrude;

#[derive(Clone, Copy, Debug)]
pub enum WidgetType {
    FileBrowser,
    MusicPlayer,
    Playlist,
}

impl FromStr for WidgetType {
    type Err = DiziError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "file_browser" => Ok(Self::FileBrowser),
            "music_player" => Ok(Self::MusicPlayer),
            "playlist" => Ok(Self::Playlist),
            s => Err(DiziError::new(
                DiziErrorKind::ParseError,
                format!("Unknown widget type: '{}'", s),
            )),
        }
    }
}

#[derive(Clone, Debug)]
pub enum LayoutComposition {
    Simple {
        widget: WidgetType,
        ratio: usize,
        border: bool,
        title: bool,
    },
    Composite {
        direction: Direction,
        widgets: Vec<LayoutComposition>,
        ratio: usize,
    },
}

impl LayoutComposition {
    pub fn ratio(&self) -> usize {
        match self {
            LayoutComposition::Simple { ratio, .. } => *ratio,
            LayoutComposition::Composite { ratio, .. } => *ratio,
        }
    }
    pub fn from(crude: &LayoutCompositionCrude) -> DiziResult<Self> {
        match crude {
            LayoutCompositionCrude::Simple {
                widget,
                ratio,
                border,
                title,
            } => {
                let widget = WidgetType::from_str(&widget)?;
                Ok(Self::Simple {
                    widget,
                    ratio: *ratio,
                    border: *border,
                    title: *title,
                })
            }
            LayoutCompositionCrude::Composite {
                direction,
                widgets,
                ratio,
            } => {
                let direction = str_to_direction(&direction)?;
                let widgets: Vec<LayoutComposition> = widgets
                    .iter()
                    .filter_map(|w| LayoutComposition::from(w).ok())
                    .collect();
                let ratio = *ratio;
                Ok(Self::Composite {
                    direction,
                    widgets,
                    ratio,
                })
            }
        }
    }
}

pub fn str_to_direction(s: &str) -> DiziResult<Direction> {
    match s {
        "horizontal" => Ok(Direction::Horizontal),
        "veritcal" => Ok(Direction::Vertical),
        s => Err(DiziError::new(
            DiziErrorKind::ParseError,
            format!("Unknown direction: '{}'", s),
        )),
    }
}
