use super::{Components, LogLevel, Style};
use crate::preludes::general::*;
use chrono::{DateTime, Local};

cfg_if! {
    if #[cfg(tui)] {
        use poise::serenity_prelude::{Guild, User, ChannelId};
    }
}

// https://docs.rs/chrono/latest/chrono/format/strftime/index.html
const TIMESTAMP_FORMAT: &str = "%d/%m/%y %r %Z";

pub struct Entry {
    pub level: LogLevel,
    pub components: Components,
    pub timestamp: DateTime<Local>,
    pub guild: Option<Guild>,
    pub user: Option<User>,
    pub channel: Option<ChannelId>,
}

unsafe impl Sync for Entry {}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

cfg_if! {
    if #[cfg(tui)] {
        use ratatui::{
            style::{Color as Colour, Modifier, Style as RatStyle},
            text::{Line, Span, Text},
            widgets::Paragraph,
        };

        impl Entry {
            fn lines(&self) -> Vec<Line> {
                vec![Line::from({
                    let mut spans = vec![
                        Span::styled(
                            self.timestamp.format(TIMESTAMP_FORMAT).to_string() + " ",
                            RatStyle::default().fg(Colour::Cyan),
                        ),
                        Span::styled(
                            format!("[{}] ", self.level.to_string()),
                            RatStyle::default()
                                .add_modifier(Modifier::BOLD)
                                .fg(match self.level {
                                    LogLevel::Info => Colour::Green,
                                    LogLevel::Warn => Colour::Yellow,
                                    LogLevel::Error => Colour::Red,
                                    LogLevel::Command => Colour::White
                                }),
                        ),
                    ];

                    spans.extend(self.components.iter().map(|(text, style)| {
                        let text = text.to_string();
                        let mut tui_style = Some(RatStyle::default());

                        // apply styles to component
                        for flag in style.iter() {
                            if let Some(x) = tui_style {
                                match flag {
                                    Style::Bold => tui_style = Some(x.add_modifier(Modifier::BOLD)),
                                    Style::Red => tui_style = Some(x.fg(Colour::Red)),
                                    Style::Green => tui_style = Some(x.fg(Colour::Green)),
                                    Style::Yellow => tui_style = Some(x.fg(Colour::Yellow)),
                                    _ => tui_style = None
                                };
                            } else {
                                break;
                            }
                        }

                        if let Some(tui_style) = tui_style {
                            Span::styled(text, tui_style)
                        } else {
                            Span::raw(text)
                        }
                    }));

                    spans
                })]
            }

            pub fn text(&self) -> Text {
                Text::from(self.lines())
            }

            pub fn paragraph(&self) -> Paragraph {
                Paragraph::new(self.text())
            }
        }
    } else {
        use owo_colors::{Stream::Stdout, OwoColorize};
        use strip_ansi::strip_ansi;

        impl ToString for Entry {
            fn to_string(&self) -> String {
                format!(
                    "{} {} {}",
                    // format timestamp
                    self.timestamp.format(TIMESTAMP_FORMAT).to_string().if_supports_color(Stdout, |x| x.cyan()),
                    // format level
                    format!("[{}]", self.level.to_string()).if_supports_color(Stdout, |x| match self.level {
                        LogLevel::Info => x.green().bold().to_string(),
                        LogLevel::Warn => x.yellow().bold().to_string(),
                        LogLevel::Error => x.red().bold().to_string(),
                        LogLevel::Command => x.bold().to_string()
                    }),
                    // build message from components
                    self
                        .components
                        .iter()
                        .map(|(text, style)| {
                            let mut text = text.to_string();

                            // apply styles to component
                            for flag in style.iter() {
                                text = text
                                    .if_supports_color(Stdout, |x| match flag {
                                        Style::Bold => x.bold().to_string(),
                                        Style::Red => x.red().to_string(),
                                        Style::Green => x.green().to_string(),
                                        Style::Yellow => x.yellow().to_string(),
                                        _ => x.to_string(),
                                    })
                                    .to_string();
                            }

                            // colour code booleans
                            text.split(" ")
                                .map(|word| {
                                    let stripped = strip_ansi(word);

                                    word.if_supports_color(Stdout, |x| match stripped.as_str() {
                                        "true" => x.green().to_string(),
                                        "false" => x.red().to_string(),
                                        _ => x.to_string(),
                                    })
                                    .to_string()
                                })
                                .collect::<Vec<String>>()
                                .join(" ")
                        })
                        .collect::<String>()
                )
            }
        }

    }
}
