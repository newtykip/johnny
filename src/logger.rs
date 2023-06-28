use crate::Context;
use chrono::{DateTime, Local};
#[cfg(not(feature = "tui"))]
use owo_colors::{
    colors::{Cyan, Green},
    OwoColorize,
    Stream::Stdout,
};
use poise::serenity_prelude::{Guild, User};
use tokio::sync::mpsc;

pub struct Logger {
    #[cfg(feature = "tui")]
    sender: Sender,
}

impl Logger {
    #[cfg(feature = "tui")]
    pub fn new(sender: Sender) -> Self {
        Self { sender }
    }

    #[cfg(not(feature = "tui"))]
    pub fn new() -> Self {
        Self {}
    }

    #[cfg(feature = "tui")]
    async fn log(&self, level: Level, message: String, ctx: Option<&Context<'_>>) {
        let entry = Entry {
            level,
            message,
            timestamp: Local::now(),
            guild: ctx.map(|ctx| ctx.guild()).flatten(),
            user: ctx.map(|ctx| ctx.author().clone()),
        };

        self.sender.send(entry).await.unwrap();
    }

    #[cfg(not(feature = "tui"))]
    async fn log(&self, level: Level, message: String) {
        let entry = Entry {
            level,
            message,
            timestamp: Local::now(),
        };

        println!("{}", entry.to_string());
    }

    #[cfg(feature = "tui")]
    pub async fn info(&self, message: String, ctx: Option<&Context<'_>>) {
        self.log(Level::Info, message, ctx).await
    }

    #[cfg(not(feature = "tui"))]
    pub async fn info(&self, message: String) {
        self.log(Level::Info, message).await
    }

    pub async fn command(&self, ctx: &Context<'_>) {
        #[cfg(feature = "tui")]
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
            Some(ctx),
        )
        .await;

        #[cfg(not(feature = "tui"))]
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
        .await;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Level {
    Info,
    Command,
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Level::Info => "INFO".to_string(),
            Level::Command => "COMMAND".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub timestamp: DateTime<Local>,
    pub level: Level,
    pub message: String,
    #[cfg(feature = "tui")]
    pub guild: Option<Guild>,
    #[cfg(feature = "tui")]
    pub user: Option<User>,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        let timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S");
        let level = format!("[{}]", self.level.to_string());

        #[cfg(not(feature = "tui"))]
        let timestamp = timestamp.if_supports_color(Stdout, |text| text.fg::<Cyan>());
        #[cfg(not(feature = "tui"))]
        let level = level.if_supports_color(Stdout, |text| match self.level {
            Level::Info => text.bold().to_string(),
            Level::Command => text.fg::<Green>().bold().to_string(),
        });

        format!("{} {} {}", timestamp, level, self.message)
    }
}

pub type Sender = mpsc::Sender<Entry>;
pub type Reciever = mpsc::Receiver<Entry>;
