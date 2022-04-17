use std::str::FromStr;

use tui::layout::Direction;

use dizi_lib::error::{DiziError, DiziErrorKind, DiziResult};

use crate::config::general::LayoutCompositionRaw;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    pub fn from(raw: &LayoutCompositionRaw) -> DiziResult<Self> {
        match raw {
            LayoutCompositionRaw::Simple {
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
            LayoutCompositionRaw::Composite {
                direction,
                widgets,
                ratio,
            } => {
                let direction = str_to_direction(&direction)?;
                let mut new_widgets: Vec<LayoutComposition> = Vec::new();
                for w in widgets {
                    let widget = LayoutComposition::from(&w)?;
                    new_widgets.push(widget);
                }

                let ratio = *ratio;
                Ok(Self::Composite {
                    direction,
                    widgets: new_widgets,
                    ratio,
                })
            }
        }
    }
}

impl std::default::Default for LayoutComposition {
    fn default() -> Self {
        LayoutComposition::Composite {
            direction: Direction::Horizontal,
            ratio: 1,
            widgets: vec![
                LayoutComposition::Simple {
                    widget: WidgetType::FileBrowser,
                    ratio: 1,
                    border: true,
                    title: true,
                },
                LayoutComposition::Composite {
                    direction: Direction::Vertical,
                    ratio: 1,
                    widgets: vec![
                        LayoutComposition::Simple {
                            widget: WidgetType::MusicPlayer,
                            ratio: 1,
                            border: true,
                            title: true,
                        },
                        LayoutComposition::Simple {
                            widget: WidgetType::Playlist,
                            ratio: 1,
                            border: true,
                            title: true,
                        },
                    ],
                },
            ],
        }
    }
}

pub fn str_to_direction(s: &str) -> DiziResult<Direction> {
    match s {
        "horizontal" => Ok(Direction::Horizontal),
        "vertical" => Ok(Direction::Vertical),
        s => Err(DiziError::new(
            DiziErrorKind::ParseError,
            format!("Unknown direction: '{}'", s),
        )),
    }
}
