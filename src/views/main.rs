use johnny::logger;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

// todo: allow logs to be dumped to a file

pub fn main<B: Backend>(
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
