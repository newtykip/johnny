mod commands;
mod events;
#[cfg(feature = "tui")]
mod tui;

use dotenvy_macro::dotenv as env;
#[cfg(feature = "johnny")]
use imgurs::ImgurClient;
#[cfg(feature = "tui")]
use johnny::Bot;
use johnny::{logger::Logger, Context, Data, Error};
#[cfg(feature = "johnny")]
use johnny::{JOHNNY_GALLERY_IDS, SUGGESTIONS_ID};
use poise::{serenity_prelude as serenity, Command, Event, Framework};
#[cfg(db)]
use sea_orm::Database;
use std::sync::Arc;

// ensure that only one of the database dirvers have been enabled
// note: this will always error in vscode as all features are enabled for intellisense, but it will compile fine
#[cfg(multiple_db)]
compile_error!("please choose only one of \"mysql\", \"sqlite\" or \"postgres\"");

// ensure that a db driver has been selected alongside any features that require a db
#[cfg(all(feature = "autorole", not(db)))]
compile_error!("please choose one of \"mysql\", \"sqlite\" or \"postgres\", you need one of them enabled for autorole to work");

macro_rules! make_feat_list {
    ($($feat:expr),*) => {
        vec![
            $(
                #[cfg(feature = $feat)]
                $feat,
            )*
        ]
    }
}

async fn start_bot(framework: Arc<Framework<Data, Error>>) {
    framework
        .start_autosharded()
        .await
        .expect("should have been able to start bot")
}

pub async fn emit_event(
    event: &Event<'_>,
    #[allow(unused_variables)] ctx: &serenity::Context,
    data: &Data,
) -> Result<(), Error> {
    match event {
        // ready
        Event::Ready { data_about_bot } => {
            #[cfg(feature = "johnny")]
            return events::ready::run(ctx, data_about_bot, data).await;
            #[cfg(not(feature = "johnny"))]
            return events::ready::run(data_about_bot, data).await;
        }

        // thread create
        #[cfg(feature = "johnny")]
        Event::ThreadCreate { thread } => {
            // suggestion created
            if thread.parent_id == Some(SUGGESTIONS_ID) {
                events::johnny::suggestion::run(ctx, thread).await
            } else {
                Ok(())
            }
        }

        _ => Ok(()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // connect to the database
    // todo: pretty error if this does not work
    #[cfg(db)]
    let _db = Database::connect(env!("DB_URL")).await?;

    #[cfg(feature = "tui")]
    let (bot, recievers) = Bot::new();

    #[cfg(feature = "tui")]
    let logger = Logger::new(bot.senders.log.clone());

    #[cfg(not(feature = "tui"))]
    let logger = Logger::new();

    // list enabled features
    let features =
        make_feat_list!["tui", "johnny", "verbose", "sqlite", "postgres", "mysql", "autorole"];

    if !features.is_empty() {
        logger
            .info(format!("Enabled features: {}", features.join(", ")), None)
            .await;
    }

    #[cfg(feature = "johnny")]
    let johnny_images = {
        let client = ImgurClient::new(&env!("IMGUR_CLIENT_ID"));
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

    #[allow(unused_mut)]
    // default commands are already in the vec
    let mut commands: Vec<Command<Data, Error>> = vec![commands::ping()];

    #[cfg(feature = "autorole")]
    commands.push(commands::autorole());

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    emit_event(event, ctx, data).await?;
                    Ok(())
                })
            },
            #[cfg(feature = "verbose")]
            post_command: |ctx| Box::pin(async move { ctx.data().logger.command(&ctx).await }),
            ..Default::default()
        })
        .token(env!("DISCORD_TOKEN"))
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

    // spawn bot
    #[cfg(feature = "tui")]
    tokio::spawn(async move { start_bot(framework).await });

    #[cfg(not(feature = "tui"))]
    start_bot(framework).await;

    // setup terminal if tui feature is enabled
    #[cfg(feature = "tui")]
    tui::prelude(recievers.log)?;

    // otherwise block the thread
    #[cfg(not(feature = "tui"))]
    loop {}

    #[allow(unreachable_code)]
    Ok(())
}
