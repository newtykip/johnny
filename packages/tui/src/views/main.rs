use super::log::State as LogState;
use crate::{helpers::generate_controls, App, Views};
use common::preludes::tui::*;

// todo: dump logs to a file
// todo: search logs
// todo: modify config file

pub struct State {
    pub following: bool,
    // stores the original following state while in a log view
    pub original_following: Option<bool>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            following: true,
            original_following: None,
        }
    }
}

pub fn controls(key_code: &KeyCode, app: &mut App, state: &mut State, log_state: &mut LogState) {
    match key_code {
        // clear logs
        KeyCode::Char('c') => {
            app.log_index = 0;
            app.logs.clear();
        }
        // toggle following and select mode
        KeyCode::Char('m') => {
            state.following = !state.following;

            if state.following {
                app.log_index = app.logs.len().saturating_sub(1);
            }
        }
        // select the log above
        KeyCode::Up => {
            if !state.following {
                app.log_index = app.log_index.saturating_sub(1)
            }
        }
        // select the log below
        KeyCode::Down => {
            if app.log_index < app.logs.len().saturating_sub(1) && !state.following {
                app.log_index += 1;
            }
        }
        // select log
        KeyCode::Enter => {
            app.view = Views::Log;
            state.original_following = Some(state.following);
            state.following = false;
            log_state.index = 0;
        }
        _ => {}
    }
}

// todo: allow logs to be dumped to a file
// todo: allow for invite generation

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App, state: &State) -> Result<()> {
    // split the screen into two vertical portions
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(f.size());

    // logs on the top
    let mut list_state = ListState::default();
    list_state.select(Some(app.log_index));

    let list = List::new(
        app.logs
            .par_iter()
            .map(|log| {
                ListItem::new(log.text()).style({
                    let mut style = Style::default();

                    if Some(log) == app.logs.get(app.log_index) {
                        style = style.bg(Color::Black);
                    }

                    style
                })
            })
            .collect::<Vec<_>>(),
    )
    .block(Block::default().borders(Borders::ALL).title("Logs"));

    f.render_stateful_widget(list, chunks[0], &mut list_state);

    // controls on the bottom
    let controls = format!(
        "Press c to clear logs, m to switch to {} mode,{} enter to select",
        if state.following {
            "select"
        } else {
            "following"
        },
        if state.following {
            ""
        } else {
            " up/down to navigate,"
        }
    );

    let controls = generate_controls(&controls);

    f.render_widget(controls, chunks[1]);

    Ok(())
}
