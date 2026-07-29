use ratatui::style::{Color, Style};

pub struct Theme;

impl Theme {
    pub const BG: Color = Color::Rgb(26, 27, 38);
    pub const BG_DARK: Color = Color::Rgb(22, 24, 33);
    pub const BG_HIGHLIGHT: Color = Color::Rgb(38, 40, 59);

    pub const FG: Color = Color::Rgb(192, 202, 245);
    pub const FG_DIM: Color = Color::Rgb(86, 95, 137);

    pub const PURPLE: Color = Color::Rgb(187, 154, 247);
    pub const PINK: Color = Color::Rgb(255, 121, 198);

    pub const BORDER: Color = Color::Rgb(59, 66, 97);

    pub fn bg() -> Style {
        Style::default().bg(Self::BG)
    }

    pub fn bg_dark() -> Style {
        Style::default().bg(Self::BG_DARK)
    }

    pub fn text() -> Style {
        Style::default().fg(Self::FG)
    }

    pub fn dim() -> Style {
        Style::default().fg(Self::FG_DIM)
    }

    pub fn purple() -> Style {
        Style::default().fg(Self::PURPLE)
    }

    pub fn pink() -> Style {
        Style::default().fg(Self::PINK)
    }

    pub fn selected() -> Style {
        Style::default().bg(Self::BG_HIGHLIGHT).fg(Self::FG)
    }

    pub fn border() -> Style {
        Style::default().fg(Self::BORDER)
    }
}
