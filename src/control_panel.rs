use crate::logger;
use crossterm::event::{self, Event, KeyCode};
use std::{
    io,
    time::{Duration, Instant},
};
use tokio::sync::oneshot;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

// todo: add event emitter submenu
// todo: add guild viewer submenu
// todo: allow logs to be viewed for extra details, including what invoked the log, who was responsible, etc

pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
    mut log_reciever: logger::Reciever,
    exit_sender: oneshot::Sender<bool>,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut logs: Vec<logger::Entry> = vec![];
    let mut selected_index = 0;

    loop {
        // draw ui
        terminal.draw(|f| logs_view(f, &mut logs.clone(), logs.get(selected_index)))?;

        // receive logs
        if let Ok(log) = log_reciever.try_recv() {
            logs.push(log);
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        exit_sender.send(true).expect("failed to send exit signal");
                        return Ok(());
                    }
                    KeyCode::Char('c') => logs.clear(),
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        let log_count = logs.len() as i32;

                        if (selected_index as i32) < log_count - 1 {
                            selected_index += 1;
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn logs_view<B: Backend>(
    f: &mut Frame<B>,
    logs: &mut [logger::Entry],
    selected_log: Option<&logger::Entry>,
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

    // and a message telling you the controls
    let controls = Paragraph::new(
        r#"Press c to clear logs
Press q to quit"#,
    )
    .style(
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::BOLD),
    )
    .block(control_block);
    f.render_widget(controls, chunks[0]);

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

            if Some(log) == selected_log {
                style = style.bg(Color::Black);
            }

            ListItem::new(format!("{} [{}] {}", log.timestamp, log.level, log.message)).style(style)
        })
        .collect::<Vec<_>>();

    let log = List::new(logs).block(log_block).highlight_symbol(">>");

    f.render_widget(log, chunks[1]);
}
