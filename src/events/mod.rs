use ::johnny::{preludes::general::*, Data};
use poise::Event;
use serenity::client::Context;

cfg_if! {
    if #[cfg(johnny)] {
        use ::johnny::SUGGESTIONS_ID;
        mod johnny;
    }
}

#[cfg(autorole)]
mod autorole;
mod general;

pub async fn event_handler(
    event: &mut Event<'_>,
    #[allow(unused_variables)] ctx: &Context,
    #[allow(unused_variables)] data: &Data,
) -> Result<()> {
    match event {
        // ready
        Event::Ready { data_about_bot } => {
            cfg_if! {
                if #[cfg(any(johnny, sqlite))] {
                    general::ready(ctx, data_about_bot).await
                } else {
                    general::ready(data_about_bot).await
                }
            }
        }

        // thread create
        #[cfg(johnny)]
        Event::ThreadCreate { thread } => {
            // suggestion created
            if thread.parent_id == Some(SUGGESTIONS_ID) {
                johnny::suggestion(ctx, thread).await
            } else {
                Ok(())
            }
        }

        // member join
        #[cfg(autorole)]
        Event::GuildMemberAddition {
            ref mut new_member, ..
        } => autorole::apply_role(ctx, new_member, &data.db).await,

        // role delete
        #[cfg(autorole)]
        Event::GuildRoleDelete {
            removed_role_id, ..
        } => autorole::role_delete(removed_role_id, &data.db).await,

        _ => Ok(()),
    }
}
