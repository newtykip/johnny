mod commands;
mod events;
#[cfg(tui)]
mod tui;

use dotenvy_macro::dotenv as env;
#[cfg(johnny)]
use imgurs::ImgurClient;
#[cfg(db)]
use johnny::db::GetDB;
use johnny::{logger::Logger, Context, Data, Error};
#[cfg(johnny)]
use johnny::{JOHNNY_GALLERY_IDS, SUGGESTIONS_ID};
#[cfg(db)]
use migration::{Migrator, MigratorTrait};
use poise::{serenity_prelude as serenity, Command, Event, Framework};
#[cfg(db)]
use sea_orm::Database;
use std::sync::Arc;
#[cfg(db)]
use std::{collections::HashSet, sync::RwLock};
#[cfg(sqlite)]
use std::{fs::File, path::Path};
#[cfg(tui)]
use tokio::sync::mpsc;

// todo: run fresh migrations on first time run

// ensure that only one of the database dirvers have been enabled
// ! this will always error in vscode as all features are enabled for intellisense, but it will compile fine
#[cfg(multiple_db)]
compile_error!("please choose only one of \"postgres\", \"mysql\" or \"sqlite\"");

// ensure that a db driver has been selected alongside any features that require a db
#[cfg(all(autorole, not(db)))]
compile_error!("please choose one of \"postgres\", \"mysql\", or \"sqlite\", you need one of them enabled for autorole to work");

macro_rules! feature_list {
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
            #[cfg(any(johnny, sqlite))]
            return events::ready::run(ctx, data_about_bot, data).await;
            #[cfg(not(any(johnny, sqlite)))]
            events::ready::run(data_about_bot, data).await
        }

        // thread create
        #[cfg(johnny)]
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
    // ensure sqlite file exists
    #[cfg(sqlite)]
    {
        let path = env!("DATABASE_URL")
            .split("://")
            .last()
            .expect("expected valid sqlite connection url");

        // allow in-memory databases (although these are absolutely NOT recommended)
        if path != ":memory:" {
            let path = Path::new(path);

            if !path.exists() {
                File::create(path)?;
            }
        }
    }

    // todo: pretty error if this does not work
    // connect to the database
    #[cfg(db)]
    let db = Database::connect(env!("DATABASE_URL")).await?;

    // run migrations
    #[cfg(db)]
    Migrator::refresh(&db).await?;

    // guild cache
    #[cfg(db)]
    let guilds_in_db = {
        let mut container = HashSet::new();

        for guild in serenity::Guild::get_db_all(&db).await? {
            container.insert(serenity::GuildId(
                guild.id.parse().expect("guild id should be a snowflake"),
            ));
        }

        container
    };

    // user cache
    #[cfg(db)]
    let users_in_db = {
        let mut container = HashSet::new();

        for user in serenity::User::get_db_all(&db).await? {
            container.insert(serenity::UserId(
                user.id.parse().expect("user id should be a snowflake"),
            ));
        }

        container
    };

    // create logger channels if applicable
    #[cfg(tui)]
    let (log_tx, log_rx) = mpsc::channel(32);

    // create logger
    let logger = if cfg!(tui) {
        Logger::new(Some(log_tx))
    } else {
        Logger::new(None)
    };

    // list enabled features
    let features =
        feature_list!["tui", "johnny", "verbose", "sqlite", "postgres", "mysql", "autorole"];

    if !features.is_empty() {
        logger
            .info(format!("Enabled features: {}", features.join(", ")), None)
            .await;
    }

    #[cfg(johnny)]
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

    // default commands are already in the vec
    #[allow(unused_mut)]
    let mut commands: Vec<Command<Data, Error>> = vec![commands::ping()];

    #[cfg(autorole)]
    commands.push(commands::autorole());

    // create the bot's framework instance
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
                            // ? log verbose?
                            id.create_db(&data.db)
                                .await
                                .expect("db connection should be active");
                            guilds.write().expect("should be writable").insert(id);
                        }
                    }

                    // ensure that the user has a document in the database
                    let user = ctx.author();
                    let users = &data.users_in_db;

                    if !users.read().expect("should be readable").contains(&user.id) {
                        // ? log verbose?
                        user.create_db(&data.db)
                            .await
                            .expect("db connection should be active");
                        users.write().expect("should be writable").insert(user.id);
                    }

                    ()
                })
            },
            #[cfg(verbose)]
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
                    #[cfg(johnny)]
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
    #[cfg(tui)]
    tokio::spawn(async move { start_bot(framework).await });

    #[cfg(not(tui))]
    start_bot(framework).await;

    // setup terminal if tui feature is enabled
    #[cfg(tui)]
    tui::prelude(log_rx)?;

    // otherwise block the thread
    #[cfg(not(tui))]
    loop {}

    #[allow(unreachable_code)]
    Ok(())
}
