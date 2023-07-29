mod build_data;
mod commands;
mod config;
mod errors;
mod events;

use common::prelude::*;
use common::Data;
use config::Config;
use errors::error_handler;
use events::event_handler;
#[cfg(johnny)]
use johnny::fetch_images;
#[cfg(tui)]
use logger::SENDER;
#[cfg(db)]
use migration::{Migrator, MigratorTrait};
use poise::samples::register_globally;
use poise::serenity_prelude::GatewayIntents;
use poise::{Framework, FrameworkOptions};
#[cfg(db)]
use sea_orm::Database;
use std::sync::Arc;
#[cfg(sqlite)]
use std::{fs::File, path::Path};
#[cfg(tui)]
use tokio::sync::mpsc;

// ensure that only one of the database drivers have been enabled
#[cfg(all(multiple_db, not(debug_assertions)))]
compile_error!("please choose only one of \"postgres\", \"mysql\" or \"sqlite\"");

// ensure that a db driver has been selected alongside any features that require a db
#[cfg(all(autorole, not(debug_assertions)))]
compile_error!("please choose one of \"postgres\", \"mysql\", or \"sqlite\", you need one of them enabled for autorole to work");

async fn start_bot(framework: Arc<Framework<Data, Error>>) -> Result<()> {
    framework
        .start_autosharded()
        .await
        .wrap_err("should have been able to start bot")
}

#[tokio::main]
async fn main() -> Result<()> {
    // set-up eyre
    color_eyre::install()?;

    // load the config
    let config = Config::load()?;

    // ensure sqlite file exists
    #[cfg(sqlite)]
    {
        let path = config
            .database
            .url
            .split("://")
            .last()
            .wrap_err("connection url must be valid")?;

        // allow in-memory databases (although these are absolutely NOT recommended)
        if path != ":memory:" {
            let path = Path::new(path);

            if !path.exists() {
                File::create(path)?;
            }
        }
    }

    // connect to the database
    #[cfg(db)]
    let db = Database::connect(config.database.url).await?;

    // apply new migrations on stable
    #[cfg(all(db, not(debug_assertions)))]
    Migrator::up(&db, None).await?;

    // reset database when dev
    #[cfg(all(db, debug_assertions))]
    Migrator::refresh(&db).await?;

    // build intents
    #[allow(unused_mut)]
    let mut intents = GatewayIntents::non_privileged();

    if cfg!(any(autorole, sticky)) {
        intents |= GatewayIntents::GUILD_MEMBERS;
    }

    // build command list
    #[allow(unused_mut)]
    let mut commands = vec![commands::ping()];

    #[cfg(autorole)]
    commands.push(autorole::autorole());

    #[cfg(sticky)]
    commands.push(sticky::sticky());

    #[cfg(pride)]
    commands.push(pride::pride());

    // create the framework
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands,
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    event_handler(&mut event.clone(), ctx, data).await?;
                    Ok(())
                })
            },
            on_error: |error| Box::pin(async move { error_handler(error).await }),
            #[cfg(verbose)]
            post_command: |ctx| {
                Box::pin(async move {
                    logger::command(&ctx).await.unwrap();
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // register application commands
                register_globally(ctx, &framework.options().commands).await?;

                Ok(Data {
                    #[cfg(johnny)]
                    johnny_images: fetch_images(&config.johnny.imgur).await?,
                    #[cfg(db)]
                    db,
                })
            })
        })
        .token(config.token)
        .intents(intents)
        .initialize_owners(true)
        .build()
        .await?;

    // spawn the bot
    #[cfg(tui)]
    {
        tokio::spawn(async move { start_bot(framework).await.unwrap() });

        // tui
        let channels = mpsc::channel(100);
        SENDER.get_or_init(|| async { channels.0 }).await;

        tui::prelude(channels.1)?;
    }

    #[cfg(not(tui))]
    {
        start_bot(framework).await?;
        loop {}
    }

    #[allow(unreachable_code)]
    Ok(())
}
