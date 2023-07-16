// todo: rewrite logger with proper logging library when tui is disabled
mod entry;
mod level;

use crate::Context;
pub use crate::Data;
pub use anyhow::{Context as AnyhowContext, Result};
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

    async fn log(
        &self,
        level: LogLevel,
        message: String,
        #[allow(unused_variables)] ctx: Option<&Context<'_>>,
    ) -> Result<()> {
        // colour code booleans
        #[cfg(tui)]
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
            #[cfg(tui)]
            guild: ctx.and_then(|ctx| ctx.guild()),
            #[cfg(tui)]
            user: ctx.map(|ctx| ctx.author().clone()),
            #[cfg(tui)]
            channel: ctx.map(|ctx| ctx.channel_id()),
        };

        if cfg!(tui) {
            self.sender
                .as_ref()
                .context("sender should exist if tui is enabled")?
                .send(entry)
                .await
                .context("should have been able to send log entry through channel to tui")?;
        } else {
            println!("{}", entry.to_string());
        }

        Ok(())
    }

    pub async fn info(&self, message: String, mut ctx: Option<&Context<'_>>) -> Result<()> {
        if cfg!(not(tui)) {
            ctx = None;
        }

        self.log(LogLevel::Info, message, ctx).await
    }

    pub async fn warn(&self, message: String, mut ctx: Option<&Context<'_>>) -> Result<()> {
        if cfg!(not(tui)) {
            ctx = None;
        }

        self.log(LogLevel::Warn, message, ctx).await
    }

    pub async fn command(&self, ctx: &Context<'_>) -> Result<()> {
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
        .await
    }
}
