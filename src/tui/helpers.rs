use once_cell::sync::Lazy;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

pub const CONTROLS_STYLE: Lazy<Style> = Lazy::new(|| {
    Style::default()
        .fg(Color::DarkGray)
        .add_modifier(Modifier::BOLD)
});

/// Generate a button with the given text and selected state
pub fn generate_button(text: &str, selected: bool) -> Paragraph {
    // format the text
    let text = format!(
        "{}{}{}",
        if selected { "[" } else { "" },
        text,
        if selected { "]" } else { "" }
    );

    // style the text
    let mut style = Style::default();

    if selected {
        style = style.fg(Color::Green);
    }

    Paragraph::new(text).style(style)
}