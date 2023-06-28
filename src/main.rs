mod commands;
mod events;
mod views;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotenvy_macro::dotenv;
#[cfg(feature = "johnny")]
use imgurs::ImgurClient;
use johnny::{logger::Logger, Bot, Context, Data, Error};
#[cfg(feature = "johnny")]
use johnny::{JOHNNY_GALLERY_ID, SUGGESTIONS_ID};
use poise::{serenity_prelude as serenity, Command, Event};
use std::{io, time::Duration};
use tokio::sync::oneshot;
use tui::{backend::CrosstermBackend, Terminal};
use views::run_tui;

pub async fn emit_event(event: &Event<'_>, ctx: &serenity::Context, data: &Data) {
    match event {
        // ready
        Event::Ready { data_about_bot } => events::ready::run(ctx, data_about_bot, data).await,

        // thread create
        #[cfg(feature = "johnny")]
        Event::ThreadCreate { thread } => {
            if thread.parent_id == Some(SUGGESTIONS_ID) {
                events::suggestion_made::run(&ctx, &thread).await;
            }
        }

        _ => {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (exit_tx, mut exit_rx) = oneshot::channel::<bool>();
    let (bot, recievers) = Bot::new();
    let logger = Logger::new(bot.senders.log.clone());

    // list enabled features
    #[allow(unused_mut)]
    let mut features: Vec<&str> = vec![];

    // ? use cfg! macro

    if !features.is_empty() {
        logger
            .info(format!("Enabled features: {}", features.join(", ")))
            .await;
    }

    #[cfg(feature = "johnny")]
    let johnny_images = ImgurClient::new(&dotenv!("IMGUR_CLIENT_ID"))
        .album_info(JOHNNY_GALLERY_ID)
        .await?
        .data
        .images
        .iter()
        .map(|image| image.link.clone())
        .filter(|link| link.ends_with(".png") || link.ends_with(".jpg"))
        .collect();

    let commands: Vec<Command<Data, Error>> = vec![commands::ping()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    emit_event(event, ctx, data).await;
                    Ok(())
                })
            },
            post_command: |ctx| Box::pin(async move { ctx.data().logger.command(&ctx).await }),
            ..Default::default()
        })
        .token(dotenv!("DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .initialize_owners(true)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    #[cfg(feature = "johnny")]
                    johnny_images,
                    logger,
                })
            })
        })
        .build()
        .await?;

    // bot thread
    tokio::spawn(async move {
        framework.start_autosharded().await.unwrap();
    });

    // tui thread
    tokio::spawn(async move {
        // setup terminal
        enable_raw_mode().expect("failed to setup terminal");
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .expect("failed to setup terminal");
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).expect("failed to setup terminal");

        // create app and run it
        let tick_rate = Duration::from_millis(250);
        let res = run_tui(&mut terminal, tick_rate, recievers.log, exit_tx);

        // restore terminal
        disable_raw_mode().expect("failed to restore terminal");
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .expect("failed to restore terminal");
        terminal.show_cursor().expect("failed to restore terminal");

        if let Err(err) = res {
            println!("{:?}", err)
        }
    });

    loop {
        if let Ok(exit) = exit_rx.try_recv() {
            if exit {
                break;
            }
        }
    }

    Ok(())
}
