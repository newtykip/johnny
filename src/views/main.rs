use super::Views;
use ansi_to_tui::IntoText;
use crossterm::event::KeyCode;
use johnny::logger::{self, LogLevel};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn controls(
    key_code: &KeyCode,
    logs: &mut Vec<logger::Entry>,
    selected_index: &mut usize,
    current_view: &mut Views,
) {
    match key_code {
        KeyCode::Char('c') => logs.clear(),
        KeyCode::Up => {
            *selected_index = selected_index.saturating_sub(1);
        }
        KeyCode::Down => {
            let item_count = logs.len() as i32;

            if (*selected_index as i32) < item_count - 1 {
                *selected_index += 1;
            }
        }
        KeyCode::Enter => {
            if !logs.is_empty() {
                *current_view = Views::Log;
            }
        }
        _ => {}
    }
}

// todo: allow logs to be dumped to a file
// todo: allow for invite generation

pub fn main<B: Backend>(f: &mut Frame<B>, logs: &mut [logger::Entry], selected_index: &usize) {
    // Split the screen into 2 vertical portions
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
        .split(f.size());

    // Logs on the top
    let log_block = Block::default()
        .borders(Borders::ALL)
        .title("Logs")
        .border_type(BorderType::Plain);

    // todo: make logs scroll
    let logs = logs
        .iter()
        .map(|log| {
            let mut style = Style::default().fg(match log.level {
                LogLevel::Info => Color::Gray,
                LogLevel::Command | LogLevel::Event => Color::DarkGray,
            });

            if Some(log) == logs.get(*selected_index) {
                style = style.bg(Color::Black);
            }

            ListItem::new(
                log.to_string()
                    .into_text()
                    .expect("log message should be convertable to text"),
            )
            .style(style)
        })
        .collect::<Vec<_>>();

    let log = List::new(logs).block(log_block).highlight_symbol(">>");

    f.render_widget(log, chunks[0]);

    // controls on the bottom
    let control_message = Paragraph::new(r#"Press c to clear logs, q to quit"#).style(
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    );

    f.render_widget(control_message, chunks[1]);
}
