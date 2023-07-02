use crate::tui::{helpers::CONTROLS_STYLE, Views};
use ansi_to_tui::IntoText;
use crossterm::event::KeyCode;
use johnny::logger;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

// todo: dump logs to a file

pub fn controls(
    key_code: &KeyCode,
    logs: &mut Vec<logger::Entry>,
    log_index: &mut usize,
    index: &mut u8,
    following: &mut bool,
    current_view: &mut Views,
) {
    match key_code {
        // clear logs
        KeyCode::Char('c') => {
            *log_index = 0;
            logs.clear();
        }
        // toggle following and select mode
        KeyCode::Char('m') => {
            *following = !*following;

            if *following {
                *log_index = logs.len().saturating_sub(1);
            }
        }
        // select the log above
        KeyCode::Up => {
            if !*following {
                *log_index = log_index.saturating_sub(1)
            }
        }
        // select the log below
        KeyCode::Down => {
            if *log_index < logs.len().saturating_sub(1) && !*following {
                *log_index += 1;
            }
        }
        // select log
        KeyCode::Enter => {
            *current_view = Views::Log;
            *following = false;
            *index = 0;
        }
        _ => {}
    }
}

// todo: allow logs to be dumped to a file
// todo: allow for invite generation

pub fn draw<B: Backend>(f: &mut Frame<B>, logs: &[logger::Entry], following: &bool, index: &usize) {
    // split the screen into two vertical portions
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(f.size());

    // logs on the top
    let mut state = ListState::default();
    state.select(Some(*index));

    let list = List::new(
        logs.iter()
            .map(|log| {
                ListItem::new(
                    log.to_string()
                        .into_text()
                        .expect("log message should be convertable to text"),
                )
                .style({
                    let mut style = Style::default();

                    if Some(log) == logs.get(*index) {
                        style = style.bg(Color::Black);
                    }

                    style
                })
            })
            .collect::<Vec<_>>(),
    )
    .block(Block::default().borders(Borders::ALL).title("Logs"));

    f.render_stateful_widget(list, chunks[0], &mut state);

    // controls on the bottom
    let controls = Paragraph::new(format!(
        r#"Press c to clear logs, m to switch to {} mode,{} enter to view, q to quit"#,
        if *following { "select" } else { "following" },
        if *following {
            ""
        } else {
            " up/down to select,"
        }
    ))
    .style(CONTROLS_STYLE.clone());

    f.render_widget(controls, chunks[1]);
}
