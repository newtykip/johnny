#![allow(clippy::single_match)]

// todo: look into porting to ratatui

mod log;
mod main;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use johnny::logger::{self, Reciever};
use log::log;
use main::main;
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

// todo: first time setup
// todo: configuration
// todo: make button highlights only appear over text

#[allow(dead_code)]
pub enum Views {
    Main,
    Log,
    Emitter,     // todo: implement
    Guild,       // todo: implement
    User,        // todo: implement
    GuildMember, // todo: implement
}

pub fn prelude(log_reciever: Reciever) {
    enable_raw_mode().expect("failed to setup terminal");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture).expect("failed to setup terminal");
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("failed to setup terminal");

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let res = run_tui(&mut terminal, tick_rate, log_reciever);

    // restore terminal
    disable_raw_mode().expect("failed to restore terminal");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .expect("failed to restore terminal");
    terminal.show_cursor().expect("failed to restore terminal");

    if let Err(err) = res {
        println!("{:?}", err)
    }
}

pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
    mut log_reciever: logger::Reciever,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut logs: Vec<logger::Entry> = vec![];
    let mut vertical_index = 0;
    let mut horizontal_index = 0;
    let mut current_view = Views::Main;

    loop {
        // draw ui
        terminal.draw(|f| match current_view {
            Views::Main => main(f, &mut logs.clone(), &mut vertical_index),
            Views::Log => log(f, &logs[vertical_index], &horizontal_index),
            _ => unimplemented!(),
        })?;

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
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }

                match current_view {
                    Views::Main => {
                        main::controls(&key.code, &mut logs, &mut vertical_index, &mut current_view)
                    }
                    Views::Log => {
                        log::controls(&key.code, &mut current_view, &mut horizontal_index)
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
