use common::{load_event, prelude::*, Data};
use poise::{serenity_prelude::Context, Event};

load_event!(ready);

pub async fn event_handler(
    event: &mut Event<'_>,
    #[allow(unused_variables)] ctx: &Context,
    #[allow(unused_variables)] data: &Data,
) -> Result<()> {
    let db = &data.db;

    match event {
        // ready
        Event::Ready { data_about_bot } => {
            ready(
                #[cfg(any(johnny, db))]
                ctx,
                data_about_bot,
                #[cfg(db)]
                db,
            )
            .await
        }

        // thread create
        #[cfg(johnny)]
        Event::ThreadCreate { thread } => {
            // suggestion created
            if thread.parent_id == Some(johnny::SUGGESTIONS_ID) {
                johnny::events::suggestion(ctx, thread).await
            } else {
                Ok(())
            }
        }

        // member join
        Event::GuildMemberAddition {
            ref mut new_member, ..
        } => {
            #[cfg(debug_assertions)]
            logger::debug(
                logger::components![
                    new_member.user.name.clone() => Red,
                    " joined " => None,
                    new_member.guild_id.name(ctx).unwrap_or("unknown guild".to_string()) => Green
                ],
                None,
            )
            .await?;

            db::events::create_user(&new_member.user, db).await?;
            db::events::create_member(new_member, db).await?;

            #[cfg(autorole)]
            autorole::events::apply_role(ctx, new_member, db).await?;

            #[cfg(sticky)]
            sticky::events::add_roles(ctx, new_member, db).await?;

            Ok(())
        }

        // member leave
        Event::GuildMemberRemoval {
            member_data_if_available,
            user,
            #[cfg(debug_assertions)]
            guild_id,
            ..
        } => {
            #[cfg(debug_assertions)]
            logger::debug(
                logger::components![
                    user.name.clone() => Red,
                    " left " => None,
                    guild_id.name(ctx).unwrap_or("unknown guild".to_string()) => Green
                ],
                None,
            )
            .await?;

            if let Some(member) = member_data_if_available {
                db::events::remove_member(member, db).await?;

                #[cfg(sticky)]
                sticky::events::save_roles(member, db).await?;
            }

            db::events::remove_user(user, db).await?;

            Ok(())
        }

        // guild create
        Event::GuildCreate { guild, .. } => db::events::create_guild(guild, db).await,

        // guild delete
        Event::GuildDelete { incomplete, .. } => db::events::remove_guild(incomplete, db).await,

        // role delete
        #[cfg(autorole)]
        Event::GuildRoleDelete {
            removed_role_id, ..
        } => autorole::events::role_delete(removed_role_id, db).await,

        Event::GuildMembersChunk { chunk } => {
            println!("{:?}", chunk);
            Ok(())
        }

        _ => Ok(()),
    }
}
