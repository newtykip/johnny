use crate::build_data::FEATURES;
use common::preludes::event::*;
#[cfg(johnny)]
use poise::serenity_prelude::Activity;
use poise::serenity_prelude::Ready;
#[cfg(db)]
use {
    db::{generate_id, guild, member, prelude::*, user},
    futures::{future, stream::StreamExt},
    itertools::Itertools,
    std::collections::HashSet,
};

pub async fn ready(
    #[cfg(any(johnny, db))] ctx: &Context,
    ready: &Ready,
    #[cfg(db)] db: &DatabaseConnection,
) -> Result<()> {
    // list enabled features
    if !FEATURES.is_empty() {
        logger::info(
            logger::components![
                "Enabled features: " => Bold,
                FEATURES.join(", ") => None
            ],
            None,
        )
        .await?;
    }

    // log that the bot is ready
    logger::info(
        logger::components![
            "Logged in as " => None,
            ready.user.name.clone() => Bold
        ],
        None,
    )
    .await?;

    // set the activity
    #[cfg(johnny)]
    ctx.set_activity(Activity::streaming(":3", "https://twitch.tv/monstercat"))
        .await;

    // ? should i move this logic to db/events?

    // ? is 100 really a sane quantity?
    #[cfg(db)]
    let guilds = ctx.cache.guilds();

    #[cfg(sqlite)]
    if guilds.len() > 100 {
        logger::warn(
            logger::components!["You are in over 100 guilds. Perhaps you should swap from sqlite?"],
            None,
        )
        .await?;
    }

    // ensure guilds have db entries
    #[cfg(db)]
    {
        // find all of the guilds/members/users in the databae
        let (db_guilds, db_members, db_users) = db
            .transaction::<_, (HashSet<String>, HashSet<String>, HashSet<String>), DbErr>(|txn| {
                Box::pin(async move {
                    let guilds = guild::Entity::find()
                        .all(txn)
                        .await?
                        .iter()
                        .map(|g| g.id.clone())
                        .collect::<HashSet<_>>();

                    println!("{:?}", guilds);

                    let members = member::Entity::find()
                        .all(txn)
                        .await?
                        .iter()
                        .map(|m| m.user_id.clone())
                        .collect::<HashSet<_>>();

                    let users = user::Entity::find()
                        .all(txn)
                        .await?
                        .iter()
                        .map(|u| u.id.clone())
                        .collect::<HashSet<_>>();

                    Ok((guilds, members, users))
                })
            })
            .await?;

        // build guild models
        let guild_models = guilds
            .iter()
            .filter(|guild| !db_guilds.contains(&guild.to_string()))
            .map(|guild| guild::ActiveModel {
                id: Set(guild.to_string()),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        // find all members
        let members = future::join_all(
            guilds
                .iter()
                .map(|g| g.members_iter(&ctx.http).collect::<Vec<_>>()),
        )
        .await;

        let members = members
            .iter()
            .flatten()
            .filter(|m| m.is_ok())
            .map(|m| m.as_ref().unwrap())
            .filter(|m| !m.user.bot);

        // build member models
        let member_models = members
            .clone()
            .filter(|m| !db_members.contains(&m.user.id.to_string()))
            .map(|m| member::ActiveModel {
                id: Set(generate_id()),
                guild_id: Set(m.guild_id.to_string()),
                user_id: Set(m.user.id.to_string()),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        // build user models
        let user_models = members
            .map(|m| m.user.id)
            .unique()
            .filter(|id| !db_users.contains(&id.to_string()))
            .map(|id| user::ActiveModel {
                id: Set(id.to_string()),
                ..Default::default()
            })
            .collect::<Vec<_>>();

        println!("{} {}", member_models.len(), user_models.len());

        guild::Entity::insert_many(guild_models).exec(db).await?;
        member::Entity::insert_many(member_models).exec(db).await?;
        user::Entity::insert_many(user_models).exec(db).await?;
    }

    Ok(())
}
