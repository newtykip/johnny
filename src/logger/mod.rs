// todo: rewrite logger with proper logging library when tui is disabled
mod entry;
mod level;

use crate::Context;
use chrono::Local;
pub use entry::Entry;
pub use level::LogLevel;
#[cfg(tui)]
use owo_colors::{
    colors::{Green, Red},
    OwoColorize,
    Stream::Stdout,
};
use tokio::sync::mpsc;

pub type Sender = mpsc::Sender<Entry>;
pub type Reciever = mpsc::Receiver<Entry>;

pub struct Logger {
    sender: Option<Sender>,
}

impl Logger {
    pub fn new(sender: Option<Sender>) -> Self {
        Self { sender }
    }

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
            channel: ctx.map(|ctx| ctx.channel_id()),
        };

        if cfg!(tui) {
            self.sender
                .as_ref()
                .expect("sender should exist if tui is enabled")
                .send(entry)
                .await
                .expect("should have been able to send log entry through channel to tui");
        } else {
            println!("{}", entry.to_string());
        }
    }

    pub async fn info(&self, message: String, mut ctx: Option<&Context<'_>>) {
        if cfg!(not(tui)) {
            ctx = None;
        }

        self.log(LogLevel::Info, message, ctx).await
    }

    pub async fn warn(&self, message: String, mut ctx: Option<&Context<'_>>) {
        if cfg!(not(tui)) {
            ctx = None;
        }

        self.log(LogLevel::Warn, message, ctx).await
    }

    pub async fn command(&self, ctx: &Context<'_>) {
        let author = ctx.author().name.clone();
        let command = ctx.command().qualified_name.clone();
        let guild = ctx.guild();
        let ctx_opt = if cfg!(tui) { Some(ctx) } else { None };

        self.log(
            LogLevel::Command,
            format!("{} ran {}", author, command)
                + &if cfg!(johnny) {
                    "".to_string()
                } else {
                    let context = if let Some(guild) = guild {
                        guild.name
                    } else {
                        "DMs".to_string()
                    };

                    format!(" in {}", context)
                },
            ctx_opt,
        )
        .await;
    }
}
