use crate::Context;
use chrono::{DateTime, Local};
#[cfg(not(feature = "tui"))]
use owo_colors::colors::Cyan;
use owo_colors::{
    colors::{Green, Red},
    OwoColorize,
    Stream::Stdout,
};
use poise::serenity_prelude::{Guild, User};
use tokio::sync::mpsc;

/// Colour code booleans
fn parse_booleans(message: String) -> String {
    message
        .replace(
            "true",
            &"true"
                .if_supports_color(Stdout, |text| text.fg::<Green>())
                .to_string(),
        )
        .replace(
            "false",
            &"false"
                .if_supports_color(Stdout, |text| text.fg::<Red>())
                .to_string(),
        )
}

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
    async fn log(&self, level: LogLevel, message: String, ctx: Option<&Context<'_>>) {
        // colour code booleans
        let message = parse_booleans(message);

        let entry = Entry {
            level,
            message,
            timestamp: Local::now(),
            guild: ctx.and_then(|ctx| ctx.guild()),
            user: ctx.map(|ctx| ctx.author().clone()),
        };

        self.sender
            .send(entry)
            .await
            .expect("should have been able to send log entry through channel to tui");
    }

    #[cfg(not(feature = "tui"))]
    async fn log(&self, level: LogLevel, message: String) {
        let entry = Entry {
            level,
            message,
            timestamp: Local::now(),
        };

        println!("{}", entry.to_string());
    }

    #[cfg(feature = "tui")]
    pub async fn info(&self, message: String, ctx: Option<&Context<'_>>) {
        self.log(LogLevel::Info, message, ctx).await
    }

    #[cfg(not(feature = "tui"))]
    pub async fn info(&self, message: String) {
        self.log(LogLevel::Info, message).await
    }

    pub async fn command(&self, ctx: &Context<'_>) {
        let author = ctx.author().name.clone();
        let command = ctx.command().name.clone();

        #[cfg(not(feature = "johnny"))]
        let context = if let Some(guild) = ctx.guild() {
            guild.name
        } else {
            "DMs".to_string()
        };

        #[cfg(feature = "tui")]
        self.log(
            LogLevel::Command,
            #[cfg(feature = "johnny")]
            format!("{} ran {}", author, command),
            #[cfg(not(feature = "johnny"))]
            format!("{} ran {} in {}", author, command, context),
            Some(ctx),
        )
        .await;

        #[cfg(not(feature = "tui"))]
        self.log(
            LogLevel::Command,
            format!("{} ran {} in {}", name, command, context),
        )
        .await;
    }

    // todo: impl
    pub async fn event() {}
}

#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Command,
    Event,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Command => "COMMAND".to_string(),
            LogLevel::Event => "EVENT".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
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
            LogLevel::Info => text.bold().to_string(),
            LogLevel::Command => text.fg::<Green>().bold().to_string(),
        });

        format!("{} {} {}", timestamp, level, self.message)
    }
}

pub type Sender = mpsc::Sender<Entry>;
pub type Reciever = mpsc::Receiver<Entry>;
