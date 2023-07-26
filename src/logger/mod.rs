mod entry;
mod level;

use crate::preludes::general::*;
use bitflags::bitflags;
pub use entry::Entry;
pub use level::LogLevel;
use tokio::sync::mpsc;

cfg_if! {
    if #[cfg(tui)] {
        use tokio::sync::OnceCell;
        pub static SENDER: OnceCell<Sender> = OnceCell::const_new();
    }
}

#[cfg(tui)]
type Sender = mpsc::Sender<Entry>;
pub type Reciever = mpsc::Receiver<Entry>;
type Components = Vec<(Box<dyn ToString + Send>, Style)>;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct Style: u8 {
        const None = 0b0000_0000;
        const Bold = 0b0000_0001;

        // colours
        const Red = 0b0000_0010;
        const Green = 0b0000_0100;
        const Yellow = 0b0000_1000;
    }
}

#[macro_export]
macro_rules! components {
    ($($message: expr)*) => {
        vec![$((Box::new($message), $crate::logger::Style::None)),*]
    };
    ($($message: expr => $($style: ident)|+),+) => {
        vec![$((Box::new($message), $($crate::logger::Style::$style)|*)),*]
    };
    ($($message: expr => $style: ident),+) => {
        vec![$((Box::new($message), $crate::logger::Style::$style)),*]
    };
}

pub mod methods {
    use super::{cfg_if, Components, Entry, LogLevel, Result};
    #[cfg(tui)]
    use super::{EyreContext, SENDER};
    use crate::Context;
    use chrono::Local;

    async fn log(level: LogLevel, components: Components, ctx: Option<&Context<'_>>) -> Result<()> {
        let entry = Entry {
            level,
            components,
            timestamp: Local::now(),
            guild: ctx.and_then(|ctx| ctx.guild()),
            user: ctx.map(|ctx| ctx.author().clone()),
            channel: ctx.map(|ctx| ctx.channel_id()),
        };

        cfg_if! {
            if #[cfg(tui)] {
                let sender = SENDER.get().unwrap().clone();

                sender
                    .send(entry)
                    .await
                    .wrap_err("channel to tui should be open")?;
            } else {
                println!("{}", entry.to_string());
            }
        }

        Ok(())
    }

    macro_rules! generate_macro {
        ($($name: ident => $level: ident)*) => {
            $(
                pub async fn $name(components: Components, ctx: Option<&Context<'_>>) -> Result<()>
                {
                    log(LogLevel::$level, components, ctx).await
                }
            )*
        }
    }

    generate_macro!(
        info => Info
        warn => Warn
        error => Error
    );

    pub async fn command(ctx: &Context<'_>) -> Result<()> {
        let author = ctx.author().name.clone();
        let command = ctx.command().qualified_name.clone();
        let where_ = if let Some(guild) = ctx.guild() {
            guild.name
        } else {
            "DMs".into()
        };
        let ctx = if cfg!(tui) { Some(ctx) } else { None };

        log(
            LogLevel::Command,
            components![
                author => Red,
                " ran " => None,
                command => None,
                " in " => None,
                where_ => Green
            ],
            ctx,
        )
        .await
    }
}
