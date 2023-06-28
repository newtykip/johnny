use std::ops::Div;

use super::Views;
use crossterm::event::KeyCode;
use johnny::logger;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

const BUTTONS: [&str; 3] = ["Guild", "Member", "User"];

// todo: add guild, user, member views

pub fn controls(key_code: &KeyCode, current_view: &mut Views, selected_index: &mut usize) {
    match key_code {
        KeyCode::Backspace => {
            *current_view = Views::Main;
        }
        KeyCode::Left => {
            if *selected_index > 0 {
                *selected_index -= 1;
            }
        }
        KeyCode::Right => {
            if *selected_index < BUTTONS.len() - 1 {
                *selected_index += 1;
            }
        }
        _ => {}
    }
}

pub fn log<B: Backend>(f: &mut Frame<B>, log: &logger::Entry, selected_index: &usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(f.size());

    let block = Block::default()
        .borders(Borders::ALL)
        .title(log.level.to_string())
        .border_type(BorderType::Plain);

    let log_text = Paragraph::new(log.to_string()).block(block.clone());

    f.render_widget(log_text, chunks[0]);

    let view_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            (0..BUTTONS.len())
                .map(|_| Constraint::Percentage(100.div(BUTTONS.len() as u16)))
                .collect::<Vec<_>>(),
        )
        .split(chunks[1]);

    for (i, label) in BUTTONS.iter().enumerate() {
        let button = Paragraph::new(*label).style({
            let mut style = Style::default();
            let is_some = match *label {
                "Guild" => log.guild.is_some(),
                "Member" => log.guild.is_some() && log.user.is_some(),
                "User" => log.user.is_some(),
                _ => false,
            };

            if is_some {
                style = style.add_modifier(Modifier::BOLD)
            } else {
                style = style.fg(Color::DarkGray)
            }

            if i == *selected_index {
                style = style.bg(Color::Black);
            }

            style
        });

        f.render_widget(button, view_chunks[i]);
    }

    let backspace = Paragraph::new("Press backspace to go back").style(
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(backspace, chunks[2]);
}
