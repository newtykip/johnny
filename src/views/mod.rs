#![allow(clippy::single_match)]

mod log;
mod main;

use crossterm::event::{self, Event, KeyCode};
use johnny::logger;
use log::log;
use main::main;
use std::{
    io,
    time::{Duration, Instant},
};
use tokio::sync::oneshot;
use tui::{backend::Backend, Terminal};

#[allow(dead_code)]
enum Views {
    Main,
    Log,
    Emitter, // todo: implement
    Guild,   // todo: implement
}

pub fn run_tui<B: Backend>(
    terminal: &mut Terminal<B>,
    tick_rate: Duration,
    mut log_reciever: logger::Reciever,
    exit_sender: oneshot::Sender<bool>,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut logs: Vec<logger::Entry> = vec![];
    let mut selected_index = 0;
    let mut current_view = Views::Main;

    loop {
        // draw ui
        terminal.draw(|f| match current_view {
            Views::Main => main(f, &mut logs.clone(), logs.get(selected_index)),
            Views::Log => log(f, &logs[selected_index]),
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
                    KeyCode::Char('q') => {
                        exit_sender.send(true).expect("failed to send exit signal");
                        return Ok(());
                    }
                    _ => {}
                }

                match current_view {
                    Views::Main => match key.code {
                        KeyCode::Char('c') => logs.clear(),
                        KeyCode::Up => {
                            selected_index = selected_index.saturating_sub(1);
                        }
                        KeyCode::Down => {
                            let log_count = logs.len() as i32;

                            if (selected_index as i32) < log_count - 1 {
                                selected_index += 1;
                            }
                        }
                        KeyCode::Enter => {
                            if !logs.is_empty() {
                                current_view = Views::Log;
                            }
                        }
                        _ => {}
                    },
                    Views::Log => match key.code {
                        KeyCode::Backspace => {
                            current_view = Views::Main;
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}
