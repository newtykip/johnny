use super::{SelectedSide, Views};
use crossterm::event::KeyCode;
use johnny::logger;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

const BUTTONS: [&str; 0] = [];

pub fn controls(
    key_code: &KeyCode,
    logs: &mut Vec<logger::Entry>,
    selected_side: &mut SelectedSide,
    selected_index: &mut usize,
    current_view: &mut Views,
) {
    match key_code {
        KeyCode::Char('c') => logs.clear(),
        KeyCode::Up => {
            *selected_index = selected_index.saturating_sub(1);
        }
        KeyCode::Down => {
            let item_count = if let SelectedSide::Controls = selected_side {
                BUTTONS.len()
            } else {
                logs.len()
            } as i32;

            if (*selected_index as i32) < item_count - 1 {
                *selected_index += 1;
            }
        }
        KeyCode::Left => {
            *selected_side = SelectedSide::Controls;
        }
        KeyCode::Right => {
            if !logs.is_empty() {
                *selected_side = SelectedSide::Logs;
            }
        }
        KeyCode::Enter => {
            if !logs.is_empty() && SelectedSide::Logs == *selected_side {
                *current_view = Views::Log;
            } else {
                match BUTTONS[*selected_index] {
                    _ => unreachable!(),
                }
            }
        }
        _ => {}
    }
}

// todo: allow logs to be dumped to a file
// todo: allow for invite generation

pub fn main<B: Backend>(
    f: &mut Frame<B>,
    logs: &mut [logger::Entry],
    selected_side: &SelectedSide,
    selected_index: &usize,
) {
    // Split the screen into 2 vertical portions
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    // Left side has buttons
    let control_block = Block::default()
        .borders(Borders::ALL)
        .title("Controls")
        .border_type(BorderType::Plain);

    let control_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[0]);

    let button_list = List::new(
        BUTTONS
            .iter()
            .enumerate()
            .map(|(i, button)| {
                let mut style = Style::default().add_modifier(Modifier::BOLD);

                if i == *selected_index && &SelectedSide::Controls == selected_side {
                    style = style.bg(Color::Black);
                }

                ListItem::new(*button).style(style)
            })
            .collect::<Vec<_>>(),
    )
    .block(control_block.clone());

    f.render_widget(button_list, control_layout[0]);

    // and a message telling you the controls
    let control_message = Paragraph::new(r#"Press c to clear logs, q to quit"#).style(
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(control_message, control_layout[1]);

    // Right side has logs
    let log_block = Block::default()
        .borders(Borders::ALL)
        .title("Logs")
        .border_type(BorderType::Plain);

    // todo: make logs scroll
    let logs = logs
        .iter()
        .map(|log| {
            let mut style = Style::default().fg(match log.level {
                logger::Level::Info => Color::Gray,
                logger::Level::Command => Color::DarkGray,
            });

            if Some(log) == logs.get(*selected_index) && &SelectedSide::Logs == selected_side {
                style = style.bg(Color::Black);
            }

            ListItem::new(log.to_string()).style(style)
        })
        .collect::<Vec<_>>();

    let log = List::new(logs).block(log_block).highlight_symbol(">>");

    f.render_widget(log, chunks[1]);
}
