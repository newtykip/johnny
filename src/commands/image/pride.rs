use image::{load_from_memory, DynamicImage, ImageError};
use johnny::{determine_avatar, preludes::command::*};
use pride_overlay::{circle as circle_pride, overlay as overlay_pride, Flags, Opacity};
use std::{borrow::Cow, io::Cursor, str::FromStr};
use strum::IntoEnumIterator;

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

/// Overlay a pride flag on an image
#[command(slash_command)]
pub async fn overlay(
    ctx: Context<'_>,
    #[description = "The flag to use"]
    #[autocomplete = "flag_autocomplete"]
    flag: String,
    #[description = "The file to apply the effect to"] attachment: Option<Attachment>,
    #[description = "The opacity of the overlay"] opacity: Option<f32>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let flag = Flags::from_str(&flag)?;
    let opacity = if let Some(o) = opacity {
        Opacity::from_percentage(o)
    } else {
        None
    };
    let image = overlay_pride(
        &mut fetch_image(&ctx, attachment).await?,
        flag.into(),
        opacity,
    );

    let mut bytes = vec![];
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

    // send the image
    ctx.send(|msg| {
        msg.attachment(AttachmentType::Bytes {
            data: Cow::Borrowed(bytes.as_slice()),
            filename: "pride.png".into(),
        })
    })
    .await?;

    Ok(())
}

/// Draw a circle with the colours of a pride flag around an image
#[command(slash_command)]
pub async fn circle(
    ctx: Context<'_>,
    #[description = "The flag to use"]
    #[autocomplete = "flag_autocomplete"]
    flag: String,
    #[description = "The file to apply the effect to"] attachment: Option<Attachment>,
    #[description = "The thickness of the ring"] thickness: Option<u8>,
) -> Result<(), Error> {
    ctx.defer().await?;

    let flag = Flags::from_str(&flag)?;
    let image = circle_pride(
        &mut fetch_image(&ctx, attachment).await?,
        flag.into(),
        thickness,
    );

    let mut bytes = vec![];
    image.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

    // send the image
    ctx.send(|msg| {
        msg.attachment(AttachmentType::Bytes {
            data: Cow::Borrowed(bytes.as_slice()),
            filename: "pride.png".into(),
        })
    })
    .await?;

    Ok(())
}

/// Apply a pride flag to an image
#[command(slash_command, subcommands("overlay", "circle"))]
pub async fn pride(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}
