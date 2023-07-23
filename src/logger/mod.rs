// todo: rewrite logger with proper logging library when tui is disabled
mod entry;
mod level;
mod style;

use crate::{preludes::general::*, Context};
use chrono::Local;
pub use entry::Entry;
use entry::Message;
pub use level::LogLevel;
pub use style::Style;
use tokio::sync::mpsc;

macro_rules! log_level {
    ($($name: ident => $level: ident)*) => {
        $(
            pub async fn $name(&self, message: Message, mut ctx: Option<&Context<'_>>) -> Result<()> {
                if cfg!(not(tui)) {
                    ctx = None;
                }

                self.log(LogLevel::$level, message, ctx).await
            }
        )*
    }
}

pub type Sender = mpsc::Sender<Entry>;
pub type Reciever = mpsc::Receiver<Entry>;

pub struct Logger {
    #[cfg(tui)]
    sender: Sender,
}

impl Logger {
    cfg_if! {
        if #[cfg(tui)] {
            pub fn new(sender: Sender) -> Self {
                Self { sender }
            }
        } else {
            pub fn new() -> Self {
                Self { }
            }
        }
    }

    async fn log(
        &self,
        level: LogLevel,
        message: Message,
        #[allow(unused_variables)] ctx: Option<&Context<'_>>,
    ) -> Result<()> {
        // todo: reimplement
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

        cfg_if! {
            if #[cfg(tui)] {
                self.sender
                    .send(entry)
                    .await
                    .wrap_err("should have been able to send log entry through channel to tui")?;
            } else {
                println!("{}", entry.to_string());
            }
        }

        Ok(())
    }

    log_level!(
        info => Info
        warn => Warn
        error => Error
    );

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
