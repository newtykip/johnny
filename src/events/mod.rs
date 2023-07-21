use ::johnny::Data;
#[cfg(johnny)]
use ::johnny::SUGGESTIONS_ID;
use anyhow::Result;
use poise::Event;
use serenity::client::Context;

mod general;
#[cfg(johnny)]
mod johnny;

pub async fn event_handler(
    event: &Event<'_>,
    #[allow(unused_variables)] ctx: &Context,
    data: &Data,
) -> Result<()> {
    match event {
        // ready
        Event::Ready { data_about_bot } => {
            #[cfg(any(johnny, sqlite))]
            return general::ready(ctx, data_about_bot, data).await;
            #[cfg(not(any(johnny, sqlite)))]
            general::ready(data_about_bot, data).await
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

        _ => Ok(()),
    }
}
