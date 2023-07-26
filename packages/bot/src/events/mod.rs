use common::{load_event, prelude::*, Data};
use poise::{serenity_prelude::Context, Event};

load_event!(ready);

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
                    ready(ctx, data_about_bot).await
                } else {
                    ready(data_about_bot).await
                }
            }
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
        #[cfg(autorole)]
        Event::GuildMemberAddition {
            ref mut new_member, ..
        } => autorole::events::apply_role(ctx, new_member, &data.db).await,

        // role delete
        #[cfg(autorole)]
        Event::GuildRoleDelete {
            removed_role_id, ..
        } => autorole::events::role_delete(removed_role_id, &data.db).await,
        _ => Ok(()),
    }
}
