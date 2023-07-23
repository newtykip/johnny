use super::LogLevel;
use super::Style;
use crate::preludes::general::*;
use chrono::{DateTime, Local};

cfg_if! {
    if #[cfg(tui)] {
        use ratatui::{
            style::{Color as RatColour, Modifier as RatModifier, Style as RatStyle},
            text::{Line, Span, Text},
            widgets::Paragraph,
        };
        use poise::serenity_prelude::{ChannelId, Guild, User};
    } else {
        use owo_colors::{OwoColorize, Stream::Stdout};
    }
}

pub type Message = Vec<(String, Option<Style>)>;

#[derive(Debug, Clone)]
pub struct Entry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: Message,
    #[cfg(tui)]
    pub guild: Option<Guild>,
    #[cfg(tui)]
    pub user: Option<User>,
    #[cfg(tui)]
    pub channel: Option<ChannelId>,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

cfg_if! {
    if #[cfg(tui)] {
        impl Entry {
            fn lines(&self) -> Vec<Line> {
                vec![Line::from({
                    let mut spans = vec![
                        Span::styled(
                            self.timestamp.to_string() + " ",
                            RatStyle::default().fg(RatColour::Cyan),
                        ),
                        Span::styled(
                            format!("[{}] ", self.level.to_string()),
                            RatStyle::default()
                                .add_modifier(RatModifier::BOLD)
                                .fg(match self.level {
                                    LogLevel::Info => RatColour::Green,
                                    LogLevel::Warn => RatColour::Yellow,
                                    LogLevel::Command => RatColour::White,
                                    LogLevel::Error => RatColour::Red,
                                }),
                        ),
                    ];

                    spans.extend(self.message.iter().map(|(content, style)| {
                        if let Some(style) = style {
                            Span::styled(content, (*style).into())
                        } else {
                            Span::raw(content)
                        }
                    }));

                    spans
                })]
            }

            pub fn text(&self) -> Text {
                Text::from(self.lines())
            }

            pub fn paragraph(&self) -> Paragraph {
                Paragraph::new(self.lines())
            }
        }
    } else {
        impl ToString for Entry {
            fn to_string(&self) -> String {
                [
                    self.timestamp
                        .if_supports_color(Stdout, |text| text.cyan())
                        .to_string(),
                    format!("[{}]", self.level.to_string())
                        .if_supports_color(Stdout, |text| match self.level {
                            LogLevel::Info => text.green().bold().to_string(),
                            LogLevel::Warn => text.yellow().bold().to_string(),
                            LogLevel::Command => text.white().bold().to_string(),
                            LogLevel::Error => text.red().bold().to_string()
                        })
                        .to_string(),
                    self.message
                        .iter()
                        .map(|(content, style)| {
                            if let Some(style) = style {
                                content
                                    .if_supports_color(Stdout, |text| text.style((*style).into()))
                                    .to_string()
                            } else {
                                content.clone()
                            }
                        })
                        .collect::<String>(),
                ]
                .join(" ")
            }
        }
    }
}
