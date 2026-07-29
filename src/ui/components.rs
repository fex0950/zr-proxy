use crate::ui::theme::Theme;
use ratatui::widgets::{Block, BorderType, Borders};

pub fn titled_block(title: &str) -> Block<'_> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Theme::border())
        .style(Theme::bg_dark())
}
