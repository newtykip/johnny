mod build_data;
mod commands;
mod config;
mod errors;
mod events;

use common::{prelude::*, Data};
use config::Config;
use errors::error_handler;
use events::event_handler;
#[cfg(johnny)]
use johnny::fetch_images;
use poise::{
    samples::register_globally, serenity_prelude::GatewayIntents, Framework, FrameworkOptions,
};
use std::sync::Arc;
#[cfg(sqlite)]
use std::{fs::File, path::Path};
#[cfg(db)]
use {
    common::db::{
        migrate::migrate,
        prelude::{generate_id, insert, select, Pool, SqlxBinder},
    },
    std::collections::HashSet,
};
#[cfg(tui)]
use {logger::SENDER, tokio::sync::mpsc};

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

    // create the sqlite file if it doesn't exist
    #[cfg(sqlite)]
    {
        let path = config
            .database
            .url
            .split("sqlite://")
            .skip(1)
            .next()
            .unwrap();

        if !Path::new(&path).exists() {
            File::create(path)?;
        }
    }

    // run migrations
    #[cfg(db)]
    migrate(config.database.url.clone())?;

    // connect to the database
    #[cfg(db)]
    let pool = Pool::connect(&config.database.url).await?;

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

    #[cfg(minecraft)]
    commands.push(minecraft::minecraft());

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
            #[cfg(db)]
            pre_command: |ctx| {
                Box::pin(async move {
                    let data = ctx.data();
                    let user_id = ctx.author().id;

                    // is the user in the cache? if so, return
                    if data.user_cache.contains(&user_id.0) {
                        return ();
                    }

                    // is the user in the database? if not, insert them
                    if select!(User, &ctx.data().pool, Id | user_id.to_string()).is_none() {
                        insert!(User, &ctx.data().pool, Id => user_id.to_string()).unwrap();
                    }

                    if let Some(guild_id) = ctx.guild_id() {
                        // is the member in the cache? if so, return
                        if data.member_cache.contains(&(guild_id.0, user_id.0)) {
                            return ();
                        }

                        // is the member in the database? if not, insert them
                        if select!(
                            Member,
                            &ctx.data().pool,
                            UserId | user_id.to_string(),
                            GuildId | guild_id.to_string()
                        )
                        .is_none()
                        {
                            insert!(
                                Member,
                                &ctx.data().pool,
                                Id => generate_id(),
                                UserId => user_id.to_string(),
                                GuildId => guild_id.to_string()
                            )
                            .unwrap();
                        }
                    }
                })
            },
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
                    pool,
                    #[cfg(db)]
                    user_cache: HashSet::new(),
                    #[cfg(db)]
                    member_cache: HashSet::new(),
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
