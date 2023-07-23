mod build_data;
mod commands;
mod config;
mod events;
#[cfg(tui)]
mod tui;

use build_data::FEATURES;
use config::Config;
use events::event_handler;
#[cfg(johnny)]
use imgurs::ImgurClient;
#[cfg(db)]
use johnny::db::GetDB;
#[cfg(johnny)]
use johnny::JOHNNY_GALLERY_IDS;
use johnny::{
    embed::{colours, generate_embed, set_guild_thumbnail},
    logger::{Logger, Style},
    message_embed,
    preludes::general::*,
    Data,
};
#[cfg(db)]
use migration::{Migrator, MigratorTrait};
use poise::{serenity_prelude as serenity, Command, Framework, FrameworkError};
#[cfg(db)]
use sea_orm::Database;
use std::sync::Arc;
#[cfg(db)]
use std::{collections::HashSet, sync::RwLock};
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

    // guild cache
    #[cfg(db)]
    let guilds_in_db = {
        let mut container = HashSet::new();

        for guild in serenity::Guild::get_db_all(&db).await? {
            container.insert(serenity::GuildId(
                guild
                    .id
                    .parse()
                    .wrap_err("guild id should be a snowflake")?,
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
                user.id.parse().wrap_err("user id should be a snowflake")?,
            ));
        }

        container
    };

    // member cache
    #[cfg(db)]
    let members_in_db = {
        let mut container = HashSet::new();

        for member in serenity::Member::get_db_all(&db).await? {
            container.insert((
                serenity::GuildId(
                    member
                        .guild_id
                        .parse()
                        .wrap_err("guild id should be a snowflake")?,
                ),
                serenity::UserId(
                    member
                        .user_id
                        .parse()
                        .wrap_err("user id should be a snowflake")?,
                ),
            ));
        }

        container
    };

    // create logger
    cfg_if! {
        if #[cfg(tui)] {
            let (log_tx, log_rx) = mpsc::channel(32);
            let logger = Logger::new(log_tx);
        } else {
            let logger = Logger::new();
        }
    }

    // list enabled features
    if !FEATURES.is_empty() {
        logger
            .info(
                vec![(format!("Enabled features: {}", FEATURES.join(", ")), None)],
                None,
            )
            .await?;
    }

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
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    event_handler(event, ctx, data).await?;
                    Ok(())
                })
            },
            #[cfg(db)]
            pre_command: |ctx| {
                // todo: consider doing this stuff on join
                Box::pin(async move {
                    let data = ctx.data();

                    // ensure that the guild has a document in the database
                    if let Some(id) = ctx.guild_id() {
                        let guild_cache = &data.guilds_in_db;

                        if !guild_cache
                            .read()
                            .expect("should be readable")
                            .contains(&id)
                        {
                            // ? log verbose?
                            id.create_db(&data.db)
                                .await
                                .expect("db connection should be active");
                            guild_cache.write().expect("should be writable").insert(id);
                        }
                    }

                    // ensure that the user has a document in the database
                    let user = ctx.author();
                    let user_cache = &data.users_in_db;

                    if !user_cache
                        .read()
                        .expect("should be readable")
                        .contains(&user.id)
                    {
                        // ? log verbose?
                        user.create_db(&data.db)
                            .await
                            .expect("db connection should be active");
                        user_cache
                            .write()
                            .expect("should be writable")
                            .insert(user.id);
                    }

                    // ensure that the member has a document in the database
                    if let Some(member) = ctx.author_member().await {
                        let member_cache = &data.members_in_db;

                        if !member_cache
                            .read()
                            .expect("should be readable")
                            .contains(&(member.guild_id, member.user.id))
                        {
                            // ? log verbose?
                            member
                                .create_db(&data.db)
                                .await
                                .expect("db connection should be active");
                            member_cache
                                .write()
                                .expect("should be writable")
                                .insert((member.guild_id, member.user.id));
                        }
                    }

                    ()
                })
            },
            on_error: |error| {
                Box::pin(async move {
                    match error {
                        FrameworkError::Setup { error, .. } => {
                            panic!("Failed to start bot: {:?}", error)
                        }
                        FrameworkError::Command { error, ctx } => {
                            let data = ctx.data();

                            // log the error
                            data.logger
                                .error(
                                    vec![
                                        ("Error in command ".into(), None),
                                        (
                                            ctx.command().qualified_name.clone(),
                                            Some(Style::default().bold()),
                                        ),
                                        (format!(": {}", error), None),
                                    ],
                                    Some(&ctx),
                                )
                                .await
                                .unwrap();

                            // create the embed
                            let mut base_embed = generate_embed(
                                ctx.author(),
                                ctx.author_member().await,
                                Some(colours::FAILURE),
                            );

                            if let Some(guild) = ctx.guild() {
                                set_guild_thumbnail(&mut base_embed, guild);
                            }

                            base_embed
                                .title("Error!")
                                .description(error);

                            // announce the error
                            ctx.send(|msg| msg.embed(|embed| message_embed!(embed, base_embed)))
                                .await
                                .unwrap();
                        }
                        error => {
                            if let Err(e) = poise::builtins::on_error(error).await {
                                panic!("Error while handling error: {:?}", e);
                            }
                        }
                    }
                })
            },
            #[cfg(verbose)]
            post_command: |ctx| {
                Box::pin(async move { ctx.data().logger.command(&ctx).await.unwrap() })
            },
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
                    logger,
                    #[cfg(db)]
                    db,
                    #[cfg(db)]
                    guilds_in_db: RwLock::new(guilds_in_db),
                    #[cfg(db)]
                    users_in_db: RwLock::new(users_in_db),
                    #[cfg(db)]
                    members_in_db: RwLock::new(members_in_db),
                })
            })
        })
        .build()
        .await?;

    // spawn bot
    cfg_if! {
        if #[cfg(tui)] {
            tokio::spawn(async move { start_bot(framework.clone()).await.unwrap() });
            tui::prelude(log_rx)?;
        } else {
            start_bot(framework).await?;
            loop {}
        }
    }
    #[allow(unreachable_code)]
    Ok(())
}
