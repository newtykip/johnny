use super::main::State as MainState;
use crate::tui::{
    helpers::{generate_button, generate_controls},
    App, Views,
};
use crossterm::event::KeyCode;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Default)]
pub struct State {
    pub index: u8,
}

pub fn controls(key_code: &KeyCode, app: &mut App, main_state: &mut MainState, state: &mut State) {
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
            if state.index > 0 {
                state.index = state.index.saturating_sub(1);
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

            if state.index < button_count.saturating_sub(1) {
                state.index += 1;
            }
        }
        _ => {}
    }
}

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App, state: &State) {
    // is any button going to be rendered?
    let entry = &app.logs[app.log_index];
    let guild_exists = entry.guild.is_some();
    let user_exists = entry.user.is_some();
    let member_exists = guild_exists && user_exists;
    let channel_exists = entry.channel.is_some();
    let button_exists = guild_exists || user_exists || channel_exists;

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
        let exists_count: u8 = if guild_exists { 1 } else { 0 }
            + if user_exists { 1 } else { 0 }
            + if member_exists { 1 } else { 0 }
            + if channel_exists { 1 } else { 0 };

        let button_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                (0..exists_count)
                    .map(|_| Constraint::Percentage(100 / exists_count as u16))
                    .collect::<Vec<_>>(),
            )
            .split(chunks[1]);

        let mut button_index = 0;

        if guild_exists {
            let current_index = button_index;
            let guild_button = generate_button("Guild", state.index == current_index);
            f.render_widget(guild_button, button_chunks[button_index as usize]);
            button_index += 1;
        }

        if member_exists {
            let current_index = button_index;
            let member_button = generate_button("Member", state.index == current_index);
            f.render_widget(member_button, button_chunks[button_index as usize]);
            button_index += 1;
        }

        if user_exists {
            let current_index = button_index;
            let user_button = generate_button("User", state.index == current_index);
            f.render_widget(user_button, button_chunks[button_index as usize]);
        }

        if channel_exists {
            let current_index = button_index;
            let channel_button = generate_button("Channel", state.index == current_index);
            f.render_widget(channel_button, button_chunks[button_index as usize]);
        }
    }

    // controls on the bottom
    let controls = generate_controls("Press backspace to return to the main view");

    f.render_widget(controls, chunks[if button_exists { 2 } else { 1 }]);
}
