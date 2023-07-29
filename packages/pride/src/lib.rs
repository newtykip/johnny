use common::{determine_avatar, load_command, preludes::command::*};
use image::{load_from_memory, DynamicImage, ImageError};
use pride_overlay::Flags;
use strum::IntoEnumIterator;

load_command!(circle, overlay);

async fn fetch_image(
    ctx: &Context<'_>,
    attachment: Option<Attachment>,
) -> Result<DynamicImage, ImageError> {
    let image = {
        let url = determine_avatar(ctx.author(), ctx.author_member().await);

        if let Some(attachment) = attachment {
            if attachment.height.is_some() {
                attachment.url
            } else {
                url
            }
        } else {
            url
        }
    };

    load_from_memory(
        &reqwest::get(image)
            .await
            .expect("should have been a valid url")
            .bytes()
            .await
            .expect("it would have been an image which definitely has a byte representation"),
    )
}

async fn flag_autocomplete(
    _ctx: Context<'_>,
    _partial: &str,
) -> impl Iterator<Item = AutocompleteChoice<String>> {
    Flags::iter().map(|flag| {
        let flag = flag.to_string();
        AutocompleteChoice {
            name: flag.clone(),
            value: flag,
        }
    })
}

/// Apply a pride flag to an image
#[command(slash_command, subcommands("overlay", "circle"), category = "image")]
pub async fn pride(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
