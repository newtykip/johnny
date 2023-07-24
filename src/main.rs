mod commands;
mod config;
mod errors;
mod events;
#[cfg(tui)]
mod tui;

use config::Config;
use errors::error_handler;
use events::event_handler;
#[cfg(johnny)]
use imgurs::ImgurClient;
#[cfg(johnny)]
use johnny::JOHNNY_GALLERY_IDS;
use johnny::{logger::SENDER, preludes::general::*, Data};
#[cfg(db)]
use migration::{Migrator, MigratorTrait};
use poise::{serenity_prelude as serenity, Command, Framework};
#[cfg(db)]
use sea_orm::Database;
use std::sync::Arc;
#[cfg(sqlite)]
use std::{fs::File, path::Path};
#[cfg(tui)]
use tokio::sync::mpsc;

// ensure that only one of the database drivers have been enabled
#[cfg(all(multiple_db, not(dev)))]
compile_error!("please choose only one of \"postgres\", \"mysql\" or \"sqlite\"");

// ensure that a db driver has been selected alongside any features that require a db
#[cfg(all(autorole, not(db)))]
compile_error!("please choose one of \"postgres\", \"mysql\", or \"sqlite\", you need one of them enabled for autorole to work");

async fn start_bot(framework: Arc<Framework<Data, Error>>) -> Result<()> {
    framework
        .start_autosharded()
        .await
        .wrap_err("should have been able to start bot")
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // load config
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

    // run migrations
    #[cfg(db)]
    Migrator::refresh(&db).await?;

    #[cfg(johnny)]
    let johnny_images = {
        let client = ImgurClient::new(&config.johnny.imgur);
        let mut images = vec![];

        for id in JOHNNY_GALLERY_IDS {
            images.extend(
                client
                    .album_info(id)
                    .await?
                    .data
                    .images
                    .iter()
                    .map(|image| image.link.clone())
                    .filter(|link| link.ends_with(".png") || link.ends_with(".jpg")),
            )
        }

        images
    };

    // default commands are already in the vec
    #[allow(unused_mut)]
    let mut commands: Vec<Command<Data, Error>> = vec![commands::ping()];

    #[cfg(autorole)]
    commands.push(commands::autorole());

    #[cfg(pride)]
    commands.push(commands::pride());

    // create the bot's framework instance
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    event_handler(event, ctx).await?;
                    Ok(())
                })
            },
            on_error: |error| {
                Box::pin(async move {
                    error_handler(error).await;
                })
            },
            #[cfg(verbose)]
            post_command: |ctx| Box::pin(async move { logger::command(&ctx).await.unwrap() }),
            ..Default::default()
        })
        .token(config.token)
        .intents(serenity::GatewayIntents::non_privileged())
        .initialize_owners(true)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    #[cfg(johnny)]
                    johnny_images,
                    #[cfg(db)]
                    db,
                })
            })
        })
        .build()
        .await?;

    // spawn bot
    cfg_if! {
        if #[cfg(tui)] {
            // start the bot
            tokio::spawn(async move { start_bot(framework.clone()).await.unwrap() });

            // start the tui
            let channels = mpsc::channel(100);
            SENDER.get_or_init(|| async { channels.0 }).await;

            tui::prelude(channels.1)?;
        } else {
            start_bot(framework).await?;
            loop {}
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
