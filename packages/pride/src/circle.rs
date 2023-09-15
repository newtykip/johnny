use super::{fetch_image, flag_autocomplete};
use common::command::*;
use pride_overlay::{circle as circle_pride, Flags};
use std::{borrow::Cow, io::Cursor, str::FromStr};

/// Draw a circle with the colours of a pride flag around an image
#[command(slash_command, category = "image")]
pub async fn circle(
    ctx: Context<'_>,
    #[description = "The flag to use"]
    #[autocomplete = "flag_autocomplete"]
    flag: String,
    #[description = "The file to apply the effect to"] attachment: Option<Attachment>,
    #[description = "The thickness of the ring"] thickness: Option<u8>,
) -> Result<()> {
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
