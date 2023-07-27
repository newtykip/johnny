mod entry;
mod level;

use bitflags::bitflags;
use chrono::Local;
use common::prelude::*;
use common::Context;
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
type Component = (Box<dyn ToString + Send>, Style);

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

/// Generate the macros for the different log levels
macro_rules! generate_macro {
    ($($name: ident => $level: ident)*) => {
        $(
            pub async fn $name(components: Vec<Component>, ctx: Option<&Context<'_>>) -> Result<()>
            {
                log(LogLevel::$level, components, ctx).await
            }
        )*
    }
}

/// Generate the components vec for the log macros
#[macro_export]
macro_rules! components {
    ($($message: expr)*) => {
        vec![$((Box::new($message), $crate::Style::None)),*]
    };
    ($($message: expr => $($style: ident)|+),+) => {
        vec![$((Box::new($message), $($crate::Style::$style)|*)),*]
    };
    ($($message: expr => $style: ident),+) => {
        vec![$((Box::new($message), $crate::Style::$style)),*]
    };
}

/// Log a message
async fn log(level: LogLevel, components: Vec<Component>, ctx: Option<&Context<'_>>) -> Result<()> {
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

// command level log
#[cfg(verbose)]
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

generate_macro!(
    info => Info
    warn => Warn
    error => Error
);
