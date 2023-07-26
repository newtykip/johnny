mod helpers;
mod views;

use common::preludes::tui::*;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    io::stdout,
    time::{Duration, Instant},
};
use views::{log, main};

const TICK_RATE: Duration = Duration::from_millis(250);

// todo: first time set-up
// todo: configuration
// todo: make button highlights only appear over text
// todo: add guild, user, member views
// todo: emitter

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Views {
    Main,
    Log,
}

pub struct App {
    logs: Vec<logger::Entry>,
    log_index: usize,
    view: Views,
}

impl Default for App {
    fn default() -> Self {
        Self {
            logs: vec![],
            log_index: 0,
            view: Views::Main,
        }
    }
}

pub fn prelude(log_reciever: logger::Reciever) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_tui(&mut terminal, log_reciever);

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    mut log_reciever: logger::Reciever,
) -> Result<()> {
    let mut last_tick = Instant::now();

    // states
    let mut app = App::default();
    let mut main_state = main::State::default();
    let mut log_state = log::State::default();

    loop {
        // draw ui
        terminal.draw(|f| match app.view {
            Views::Main => main::draw(f, &app, &main_state).unwrap(),
            Views::Log => log::draw(f, &app, &log_state).unwrap(),
            #[allow(unreachable_patterns)]
            _ => unimplemented!(),
        })?;

        // receive logs
        if let Ok(log) = log_reciever.try_recv() {
            app.logs.push(log);

            if app.view == Views::Main && main_state.following {
                app.log_index = app.logs.len().saturating_sub(1);
            }
        }

        // delta time
        let timeout = TICK_RATE
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
                match app.view {
                    Views::Main => {
                        main::controls(&key.code, &mut app, &mut main_state, &mut log_state)
                    }
                    Views::Log => {
                        log::controls(&key.code, &mut app, &mut main_state, &mut log_state)
                    }
                    #[allow(unreachable_patterns)]
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            last_tick = Instant::now();
        }
    }
}
