mod commands;
mod events;
#[cfg(feature = "tui")]
mod views;

use dotenvy_macro::dotenv;
#[cfg(feature = "johnny")]
use imgurs::ImgurClient;
#[cfg(feature = "tui")]
use johnny::Bot;
use johnny::{logger::Logger, Context, Data, Error};
#[cfg(feature = "johnny")]
use johnny::{JOHNNY_GALLERY_ID, SUGGESTIONS_ID, USERNAMES_ID};
use poise::{serenity_prelude as serenity, Command, Event, Framework};
use std::sync::Arc;

fn create_feature_list<'t>() -> Vec<&'t str> {
    let mut features = vec![];

    if cfg!(feature = "tui") {
        features.push("tui");
    }

    if cfg!(feature = "johnny") {
        features.push("johnny");
    }

    if cfg!(feature = "verbose") {
        features.push("verbose");
    }

    if cfg!(feature = "autorole") {
        features.push("autorole");
    }

    features
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

        // message send
        #[cfg(feature = "johnny")]
        Event::Message { new_message } => {
            // username posted
            if new_message.channel_id == USERNAMES_ID {
                events::johnny::single_username::run(ctx, new_message).await
            } else {
                Ok(())
            }
        }

        _ => Ok(()),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[cfg(feature = "tui")]
    let (bot, recievers) = Bot::new();

    #[cfg(feature = "tui")]
    let logger = Logger::new(bot.senders.log.clone());

    #[cfg(not(feature = "tui"))]
    let logger = Logger::new();

    // list enabled features
    let features = create_feature_list();

    if !features.is_empty() {
        logger
            .info(format!("Enabled features: {}", features.join(", ")), None)
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

    // spawn bot
    #[cfg(feature = "tui")]
    tokio::spawn(async move { start_bot(framework).await });

    #[cfg(not(feature = "tui"))]
    start_bot(framework).await;

    // setup terminal if tui feature is enabled
    #[cfg(feature = "tui")]
    views::prelude(recievers.log);

    // otherwise block the thread
    #[cfg(not(feature = "tui"))]
    loop {}

    #[allow(unreachable_code)]
    Ok(())
}
