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

pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
    mut log_reciever: logger::Reciever,
    exit_sender: oneshot::Sender<bool>,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut logs: Vec<logger::Entry> = vec![];

    loop {
        // draw ui
        terminal.draw(|f| ui(f, &mut logs))?;

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
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, logs: &mut Vec<logger::Entry>) {
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
    let controls = Paragraph::new("Press q to quit")
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
        .map(|log| ListItem::new(log.message.clone()))
        .collect::<Vec<_>>();

    let log = List::new(logs).block(log_block).highlight_symbol(">>");

    f.render_widget(log, chunks[1]);
}
