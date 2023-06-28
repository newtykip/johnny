use crate::Context;
use chrono::{DateTime, Local};
use std::fmt::Display;
use tokio::sync::mpsc;

pub struct Logger {
    sender: Sender,
}

impl Logger {
    pub fn new(sender: Sender) -> Self {
        Self { sender }
    }

    async fn log(&self, level: Level, message: String) {
        self.sender
            .send(Entry {
                level,
                message,
                timestamp: Local::now(),
            })
            .await
            .unwrap()
    }

    pub async fn info(&self, message: String) {
        self.log(Level::Info, message).await
    }

    pub async fn command(&self, ctx: &Context<'_>) {
        self.log(
            Level::Command,
            format!(
                "{} ran {} in {}",
                ctx.author().name,
                ctx.command().name,
                if let Some(guild) = ctx.guild() {
                    guild.name
                } else {
                    "DMs".to_string()
                }
            ),
        )
        .await
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Level {
    Info,
    Command,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Info => write!(f, "INFO"),
            Level::Command => write!(f, "COMMAND"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    pub timestamp: DateTime<Local>,
    pub level: Level,
    pub message: String,
}

pub type Sender = mpsc::Sender<Entry>;
pub type Reciever = mpsc::Receiver<Entry>;
