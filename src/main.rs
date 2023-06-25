use dotenvy_macro::dotenv;
use imgurs::ImgurClient;
use johnny::{Context, Data, Error, JOHNNY_GALLERY_ID, SUGGESTIONS_ID};
use poise::{serenity_prelude as serenity, Event};

mod commands;
mod events;

pub async fn emit_event(event: &Event<'_>, ctx: &serenity::Context, data: &Data) {
    match event {
        // ready
        Event::Ready { data_about_bot } => events::ready::run(&ctx, &data_about_bot).await,

        // thread create
        Event::ThreadCreate { thread } => {
            if thread.parent_id == Some(SUGGESTIONS_ID) {
                events::suggestion_made::run(&ctx, &thread).await;
            }
        }

        // member join
        Event::GuildMemberAddition { new_member } => {
            events::member_join::run(&ctx, &new_member, &data).await
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::meow(), commands::emit()],
            event_handler: |ctx, event, _framework, data| {
                Box::pin(async move {
                    emit_event(&event, &ctx, &data).await;
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(dotenv!("DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    // fetch johnny images from imgur
                    johnny_images: ImgurClient::new(&dotenv!("IMGUR_CLIENT_ID"))
                        .album_info(JOHNNY_GALLERY_ID)
                        .await?
                        .data
                        .images
                        .iter()
                        .map(|image| image.link.clone())
                        .filter(|link| link.ends_with(".png") || link.ends_with(".jpg"))
                        .collect(),
                })
            })
        })
        .build()
        .await?
        .start_autosharded()
        .await?;

    Ok(())
}
