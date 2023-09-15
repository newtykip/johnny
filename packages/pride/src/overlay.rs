use super::{fetch_image, flag_autocomplete};
use common::command::*;
use pride_overlay::{overlay as overlay_pride, Flags, Opacity};
use std::{borrow::Cow, io::Cursor, str::FromStr};

/// Overlay a pride flag on an image
#[command(slash_command, category = "image")]
pub async fn overlay(
    ctx: Context<'_>,
    #[description = "The flag to use"]
    #[autocomplete = "flag_autocomplete"]
    flag: String,
    #[description = "The file to apply the effect to"] attachment: Option<Attachment>,
    #[description = "The opacity of the overlay"] opacity: Option<f32>,
) -> Result<()> {
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
