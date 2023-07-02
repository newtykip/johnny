use super::main::MainState;
use crate::tui::{
    helpers::{generate_button, CONTROLS_STYLE},
    App, Views,
};
use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct LogState {
    pub index: u8,
}

impl Default for LogState {
    fn default() -> Self {
        Self { index: 0 }
    }
}

pub fn controls(
    key_code: &KeyCode,
    app: &mut App,
    main_state: &mut MainState,
    log_state: &mut LogState,
) {
    match key_code {
        // go back to main view
        KeyCode::Backspace => {
            app.view = Views::Main;

            if let Some(original_following) = main_state.original_following {
                main_state.following = original_following;
                main_state.original_following = None;
            }
        }
        // select button to the left
        KeyCode::Left => {
            if log_state.index > 0 {
                log_state.index = log_state.index.saturating_sub(1);
            }
        }
        // select button to the right
        KeyCode::Right => {
            let mut button_count: u8 = 0;
            let entry = &app.logs[app.log_index];

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

            if log_state.index < button_count.saturating_sub(1) {
                log_state.index += 1;
            }
        }
        _ => {}
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App, state: &LogState) {
    // is any button going to be rendered?
    let entry = &app.logs[app.log_index];
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
            let guild_button = generate_button("Guild", state.index == current_index);
            f.render_widget(guild_button, button_chunks[button_index as usize]);
            button_index += 1;
        }

        if guild_exists && user_exists {
            let current_index = button_index.clone();
            let member_button = generate_button("Member", state.index == current_index);
            f.render_widget(member_button, button_chunks[button_index as usize]);
            button_index += 1;
        }

        if user_exists {
            let current_index = button_index.clone();
            let user_button = generate_button("User", state.index == current_index);
            f.render_widget(user_button, button_chunks[button_index as usize]);
        }
    }

    // controls on the bottom
    let controls = Paragraph::new("Press backspace to return to the main view, q to quit")
        .style(CONTROLS_STYLE.clone());

    f.render_widget(controls, chunks[if button_exists { 2 } else { 1 }]);
}
