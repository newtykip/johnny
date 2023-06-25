use dotenvy_macro::dotenv;
use imgurs::ImgurClient;
use johnny::{Context, Data, Error};
use johnny::{JOHNNY_GALLERY_ID, SUGGESTIONS_ID};
use poise::serenity_prelude as serenity;
use poise::Event;

mod commands;
mod events;

/// Displays your or another user's account creation date
// #[poise::command(slash_command, prefix_command)]
// async fn age(
//     ctx: Context<'_>,
//     #[description = "Selected user"] user: Option<serenity::User>,
// ) -> Result<(), Error> {
//     let u = user.as_ref().unwrap_or_else(|| ctx.author());
//     let response = format!("{}'s account was created at {}", u.name, u.created_at());
//     ctx.say(response).await?;
//     Ok(())
// }

#[tokio::main]
async fn main() {
    poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::meow()],
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event {
                        // ready
                        Event::Ready { data_about_bot } => {
                            events::ready::run(&ctx, &data_about_bot).await;
                        }

                        // thread create
                        Event::ThreadCreate { thread } => {
                            if thread.parent_id == Some(SUGGESTIONS_ID) {
                                events::suggestion_made::run(&ctx, &thread).await;
                            }
                        }
                        _ => {}
                    }

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
        .run()
        .await
        .unwrap()
}
