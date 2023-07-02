#![allow(clippy::single_match)]

mod helpers;
mod views;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use johnny::logger::{self, Reciever};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use views::{log, main};

// todo: first time setup
// todo: configuration
// todo: make button highlights only appear over text
// todo: add guild, user, member views

#[allow(dead_code)]
pub enum Views {
    Main,
    Log,
}

pub fn prelude(log_reciever: Reciever) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let res = run_tui(&mut terminal, tick_rate, log_reciever);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
    mut log_reciever: logger::Reciever,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut logs: Vec<logger::Entry> = vec![];
    let mut current_view = Views::Main;

    // general usage
    let mut index = 0;

    // main view
    let mut log_index = 0;
    let mut following = true;

    loop {
        // draw ui
        terminal.draw(|f| match current_view {
            Views::Main => main::draw(f, &logs, &following, &log_index),
            Views::Log => log::draw(f, &logs[log_index], &index),
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        })?;

        // receive logs
        if let Ok(log) = log_reciever.try_recv() {
            logs.push(log);

            if following {
                log_index = logs.len().saturating_sub(1);
            }
        }

        // delta time
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // default keybinds
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }

                // view specific keybinds
                match current_view {
                    Views::Main => main::controls(
                        &key.code,
                        &mut logs,
                        &mut log_index,
                        &mut index,
                        &mut following,
                        &mut current_view,
                    ),
                    Views::Log => log::controls(
                        &key.code,
                        &mut index,
                        &logs[log_index],
                        &mut following,
                        &mut current_view,
                    ),
                    #[allow(unreachable_patterns)]
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
