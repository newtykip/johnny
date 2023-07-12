mod commands;
mod events;
#[cfg(feature = "tui")]
mod tui;

use dotenvy_macro::dotenv as env;
#[cfg(db)]
use entity::{guild::Entity as Guild, user::Entity as User};
#[cfg(feature = "johnny")]
use imgurs::ImgurClient;
#[cfg(db)]
use johnny::db::{create_guild, create_user};
#[cfg(feature = "tui")]
use johnny::Bot;
use johnny::{logger::Logger, Context, Data, Error};
#[cfg(feature = "johnny")]
use johnny::{JOHNNY_GALLERY_IDS, SUGGESTIONS_ID};
#[cfg(db)]
use migration::{Migrator, MigratorTrait};
use poise::{serenity_prelude as serenity, Command, Event, Framework};
#[cfg(db)]
use sea_orm::{Database, EntityTrait};
use std::sync::Arc;
#[cfg(db)]
use std::{collections::HashSet, sync::RwLock};

// todo: run fresh migrations on first time run

// ensure that only one of the database dirvers have been enabled
// ! this will always error in vscode as all features are enabled for intellisense, but it will compile fine
#[cfg(multiple_db)]
compile_error!("please choose only one of \"postgres\", \"mysql\" or \"sqlite\"");

// ensure that a db driver has been selected alongside any features that require a db
#[cfg(all(feature = "autorole", not(db)))]
compile_error!("please choose one of \"postgres\", \"mysql\", or \"sqlite\", you need one of them enabled for autorole to work");

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
            #[cfg(any(feature = "johnny", feature = "sqlite"))]
            return events::ready::run(ctx, data_about_bot, data).await;
            #[cfg(not(any(feature = "johnny", feature = "sqlite")))]
            events::ready::run(data_about_bot, data).await
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
    // todo: ensure sqlite file exists
    #[cfg(db)]
    let db = Database::connect(env!("DATABASE_URL")).await?;

    // run migrations
    #[cfg(db)]
    Migrator::refresh(&db).await?;

    // make a cache of all of the guilds that have documents in the database
    #[cfg(db)]
    let guilds_in_db = {
        let mut container = HashSet::new();

        Guild::find().all(&db).await?.iter().for_each(|guild| {
            container.insert(serenity::GuildId(
                guild.id.parse().expect("guild id should be a snowflake"),
            ));
        });

        container
    };

    // do the same for users
    #[cfg(db)]
    let users_in_db = {
        let mut container = HashSet::new();

        User::find().all(&db).await?.iter().for_each(|user| {
            container.insert(serenity::UserId(
                user.id.parse().expect("user id should be a snowflake"),
            ));
        });

        container
    };

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
            #[cfg(db)]
            pre_command: |ctx| {
                Box::pin(async move {
                    let data = ctx.data();

                    // ensure that the guild has a document in the database
                    if let Some(id) = ctx.guild_id() {
                        let guilds = &data.guilds_in_db;

                        if !guilds.read().expect("should be readable").contains(&id) {
                            Guild::insert(create_guild(id))
                                .exec(&data.db)
                                .await
                                .expect("should be able to insert guild if connection is active");

                            guilds.write().expect("should be writable").insert(id);
                        }
                    }

                    // ensure that the user has a document in the database
                    let author_id = ctx.author().id;
                    let users = &data.users_in_db;

                    if !users
                        .read()
                        .expect("should be readable")
                        .contains(&author_id)
                    {
                        println!("{:?}", ctx.author().name);

                        User::insert(create_user(author_id))
                            .exec(&data.db)
                            .await
                            .expect("should be able to insert user if connection is active");

                        users.write().expect("should be writable").insert(author_id);
                    }

                    ()
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
                    #[cfg(db)]
                    db,
                    #[cfg(db)]
                    guilds_in_db: RwLock::new(guilds_in_db),
                    #[cfg(db)]
                    users_in_db: RwLock::new(users_in_db),
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
