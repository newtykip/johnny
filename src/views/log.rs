use crossterm::event::KeyCode;
use johnny::logger;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::Views;

// todo: add details

pub fn controls(key_code: &KeyCode, current_view: &mut Views) {
    match key_code {
        KeyCode::Backspace => {
            *current_view = Views::Main;
        }
        _ => {}
    }
}

pub fn log<B: Backend>(f: &mut Frame<B>, log: &logger::Entry) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(45),
            Constraint::Percentage(15),
        ])
        .split(f.size());

    let block = Block::default()
        .borders(Borders::ALL)
        .title(log.level.to_string())
        .border_type(BorderType::Plain);

    let log = Paragraph::new(log.to_string()).block(block);

    f.render_widget(log, chunks[0]);

    let backspace = Paragraph::new("Press backspace to go back").style(
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(backspace, chunks[2]);
}
