// todo: rewrite logger with proper logging library when tui is disabled
mod entry;
mod level;

use crate::Context;
use chrono::Local;
pub use entry::Entry;
pub use level::LogLevel;
#[cfg(feature = "tui")]
use owo_colors::{
    colors::{Green, Red},
    OwoColorize,
    Stream::Stdout,
};
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
    async fn log(&self, level: LogLevel, message: String, ctx: Option<&Context<'_>>) {
        // colour code booleans
        let message = message
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
            );

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
    async fn log(&self, level: LogLevel, message: String, _ctx: Option<&Context<'_>>) {
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
    pub async fn info(&self, message: String, _ctx: Option<&Context<'_>>) {
        self.log(LogLevel::Info, message, None).await
    }

    pub async fn warn(&self, message: String, ctx: Option<&Context<'_>>) {
        self.log(LogLevel::Warn, message, ctx).await
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
            format!("{} ran {} in {}", author, command, context),
            None,
        )
        .await;
    }
}

pub type Sender = mpsc::Sender<Entry>;
pub type Reciever = mpsc::Receiver<Entry>;
