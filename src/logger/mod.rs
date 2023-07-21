// todo: rewrite logger with proper logging library when tui is disabled
mod entry;
mod level;
mod style;

use crate::{preludes::eyre::*, Context};
use chrono::Local;
pub use entry::Entry;
use entry::Message;
pub use level::LogLevel;
pub use style::Style;
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
        message: Message,
        #[allow(unused_variables)] ctx: Option<&Context<'_>>,
    ) -> Result<()> {
        // // colour code booleans
        // #[cfg(tui)]
        // let message = message
        //     .replace(
        //         "true",
        //         &"true"
        //             .if_supports_color(Stdout, |text| text.fg::<Green>())
        //             .to_string(),
        //     )
        //     .replace(
        //         "false",
        //         &"false"
        //             .if_supports_color(Stdout, |text| text.fg::<Red>())
        //             .to_string(),
        //     );

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

        #[cfg(tui)]
        self.sender
            .as_ref()
            .wrap_err("sender should exist if tui is enabled")?
            .send(entry)
            .await
            .wrap_err("should have been able to send log entry through channel to tui")?;

        #[cfg(not(tui))]
        println!("{}", entry.to_string());

        Ok(())
    }

    pub async fn info(&self, message: Message, mut ctx: Option<&Context<'_>>) -> Result<()> {
        if cfg!(not(tui)) {
            ctx = None;
        }

        self.log(LogLevel::Info, message, ctx).await
    }

    pub async fn warn(&self, message: Message, mut ctx: Option<&Context<'_>>) -> Result<()> {
        if cfg!(not(tui)) {
            ctx = None;
        }

        self.log(LogLevel::Warn, message, ctx).await
    }

    pub async fn command(&self, ctx: &Context<'_>) -> Result<()> {
        let author = ctx.author().name.clone();
        let command = ctx.command().qualified_name.clone();
        let guild = ctx.guild();
        let ctx = if cfg!(tui) { Some(ctx) } else { None };

        self.log(
            LogLevel::Command,
            vec![(
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
                None,
            )],
            ctx,
        )
        .await
    }
}
