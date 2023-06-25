use crate::{emit_event, Context, Error};
use poise::{ChoiceParameter, Event};
use rand::seq::SliceRandom;

#[derive(ChoiceParameter)]
pub enum ImplementedEvents {
    #[name = "Thread Create"]
    ThreadCreate,
    #[name = "Member Join"]
    GuildMemberAddition,
}

impl ImplementedEvents {
    pub fn to_event(&self, ctx: &Context<'_>) -> Event<'_> {
        let serenity_ctx = ctx.serenity_context();

        // find channels
        let guilds = serenity_ctx.cache.guilds();

        let guild_channels = guilds
            .iter()
            .map(|id| serenity_ctx.cache.guild_channels(id))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap());

        let thread_channels = guild_channels
            .flatten()
            .map(|(_, c)| c)
            .filter(|c| c.thread_metadata.is_some())
            .collect::<Vec<_>>();

        // find members
        let members = serenity_ctx
            .cache
            .users()
            .iter()
            .map(|u| u.id)
            .zip(serenity_ctx.cache.guilds())
            .map(|(user_id, guild_id)| serenity_ctx.cache.member(guild_id, user_id))
            .filter(|m| m.is_some())
            .map(|m| m.unwrap())
            .collect::<Vec<_>>();

        match self {
            ImplementedEvents::ThreadCreate => Event::ThreadCreate {
                thread: thread_channels
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone(),
            },
            ImplementedEvents::GuildMemberAddition => Event::GuildMemberAddition {
                new_member: members.choose(&mut rand::thread_rng()).unwrap().clone(),
            },
        }
    }
}

// todo: make slash command
#[poise::command(prefix_command)]
pub async fn emit(
    ctx: Context<'_>,
    #[description = "The event you would like to emit"] event: ImplementedEvents,
) -> Result<(), Error> {
    emit_event(&event.to_event(&ctx), &ctx.serenity_context(), &ctx.data()).await;
    Ok(())
}
