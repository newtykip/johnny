use crate::tui::{
    helpers::{generate_button, CONTROLS_STYLE},
    Views,
};
use crossterm::event::KeyCode;
use johnny::logger;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn controls(
    key_code: &KeyCode,
    index: &mut u8,
    entry: &logger::Entry,
    following: &mut bool,
    current_view: &mut Views,
) {
    match key_code {
        // go back to main view
        KeyCode::Backspace => {
            *current_view = Views::Main;
            *following = true;
        }
        // select button to the left
        KeyCode::Left => {
            if *index > 0 {
                *index -= 1;
            }
        }
        // select button to the right
        KeyCode::Right => {
            let mut button_count = 0;

            // guild button
            if entry.guild.is_some() {
                button_count += 1;
            }

            // member button
            if entry.guild.is_some() && entry.user.is_some() {
                button_count += 1;
            }

            // user button
            if entry.user.is_some() {
                button_count += 1;
            }

            if *index < button_count - 1 {
                *index += 1;
            }
        }
        _ => {}
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, entry: &logger::Entry, index: &u8) {
    // is any button going to be rendered?
    let guild_exists = entry.guild.is_some();
    let user_exists = entry.user.is_some();
    let button_exists = guild_exists || user_exists;

    // split the screen into two/three vertical portions
    let contraints = if button_exists {
        vec![
            Constraint::Percentage(50), // todo: base on the length of the log message
            Constraint::Percentage(40),
            Constraint::Percentage(10),
        ]
    } else {
        vec![
            Constraint::Percentage(60), // todo: base on the length of the log message
            Constraint::Percentage(40),
        ]
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(contraints)
        .split(f.size());

    // log on the top
    let log = Paragraph::new(entry.to_string()).block(
        Block::default()
            .borders(Borders::ALL)
            .title(entry.level.to_string()),
    );

    f.render_widget(log, chunks[0]);

    // buttons in the middle
    if button_exists {
        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(chunks[1]);

        let mut button_index = 0;

        if guild_exists {
            let current_index = button_index.clone();
            let guild_button = generate_button("Guild", *index == current_index);
            f.render_widget(guild_button, button_chunks[button_index as usize]);
            button_index += 1;
        }

        if guild_exists && user_exists {
            let current_index = button_index.clone();
            let member_button = generate_button("Member", *index == current_index);
            f.render_widget(member_button, button_chunks[button_index as usize]);
            button_index += 1;
        }

        if user_exists {
            let current_index = button_index.clone();
            let user_button = generate_button("User", *index == current_index);
            f.render_widget(user_button, button_chunks[button_index as usize]);
        }
    }

    // controls on the bottom
    let controls = Paragraph::new("Press backspace to return to the main view, q to quit")
        .style(CONTROLS_STYLE.clone());

    f.render_widget(controls, chunks[if button_exists { 2 } else { 1 }]);
}
