use serde_derive::Deserialize;

use tui::style;

const fn default_color() -> style::Color {
    style::Color::Reset
}

#[derive(Clone, Debug, Deserialize)]
pub struct AppStyleRaw {
    #[serde(default)]
    pub fg: String,
    #[serde(default)]
    pub bg: String,
    #[serde(default)]
    pub bold: bool,
    #[serde(default)]
    pub underline: bool,
    #[serde(default)]
    pub invert: bool,
}

impl AppStyleRaw {
    pub fn to_style_theme(&self) -> AppStyle {
        let bg = Self::str_to_color(self.bg.as_str());
        let fg = Self::str_to_color(self.fg.as_str());

        let mut modifier = style::Modifier::empty();
        if self.bold {
            modifier.insert(style::Modifier::BOLD);
        }
        if self.underline {
            modifier.insert(style::Modifier::UNDERLINED);
        }
        if self.invert {
            modifier.insert(style::Modifier::REVERSED);
        }

        AppStyle::default().set_fg(fg).set_bg(bg).insert(modifier)
    }

    pub fn str_to_color(s: &str) -> style::Color {
        match s {
            "black" => style::Color::Black,
            "red" => style::Color::Red,
            "green" => style::Color::Green,
            "yellow" => style::Color::Yellow,
            "blue" => style::Color::Blue,
            "magenta" => style::Color::Magenta,
            "cyan" => style::Color::Cyan,
            "gray" => style::Color::Gray,
            "dark_gray" => style::Color::DarkGray,
            "light_red" => style::Color::LightRed,
            "light_green" => style::Color::LightGreen,
            "light_yellow" => style::Color::LightYellow,
            "light_blue" => style::Color::LightBlue,
            "light_magenta" => style::Color::LightMagenta,
            "light_cyan" => style::Color::LightCyan,
            "white" => style::Color::White,
            "reset" => style::Color::Reset,
            _s => style::Color::Reset,
        }
    }
}

impl std::default::Default for AppStyleRaw {
    fn default() -> Self {
        Self {
            bg: "".to_string(),
            fg: "".to_string(),
            bold: false,
            underline: false,
            invert: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppStyle {
    pub fg: style::Color,
    pub bg: style::Color,
    pub modifier: style::Modifier,
}

impl AppStyle {
    pub fn set_bg(mut self, bg: style::Color) -> Self {
        self.bg = bg;
        self
    }
    pub fn set_fg(mut self, fg: style::Color) -> Self {
        self.fg = fg;
        self
    }

    pub fn insert(mut self, modifier: style::Modifier) -> Self {
        self.modifier.insert(modifier);
        self
    }
}

impl std::default::Default for AppStyle {
    fn default() -> Self {
        Self {
            fg: default_color(),
            bg: default_color(),
            modifier: style::Modifier::empty(),
        }
    }
}
