use bytes::Bytes;
use common::command::*;
use std::borrow::Cow;

// todo: no longer rely on pixelstacker/self host api, generate schematic + block list

async fn pixel_art(
    url: String,
    height: Option<u16>,
    width: Option<u16>,
    multi: Option<bool>,
    dithering: Option<bool>,
) -> Result<Bytes> {
    // resolve options
    let mut height = height.unwrap_or(200);
    let mut width = width.unwrap_or(200);
    let multi = multi.unwrap_or(true);
    let dithering = dithering.unwrap_or(false);

    // cap height and width at 4k (limit for pixelstacker)
    if height > 4000 {
        height = 4000;
    }

    if width > 4000 {
        width = 4000;
    }

    reqwest::get(format!("https://taylorlove.info/projects/pixelstacker/api/Render/ByUrlAdvanced?Url={url}&Format=Png&IsMultiLayer={multi}&MaxHeight={height}&MaxWidth={width}&EnableDithering={dithering}")).await?.bytes().await.wrap_err("Failed to fetch pixel art")
}

/// Generate Minecraft pixel art from an image
#[command(slash_command, category = "minecraft")]
pub async fn image(
    ctx: Context<'_>,
    #[description = "The image to create a pixel art from"] attachment: Attachment,
    #[description = "The max height of the pixel art"] height: Option<u16>,
    #[description = "The max width of the pixel art"] width: Option<u16>,
    #[description = "Allow for multiple layers to be generated"] multi: Option<bool>,
    #[description = "Enable dithering"] dithering: Option<bool>,
) -> Result<()> {
    ctx.defer().await?;
    let art = pixel_art(attachment.url, height, width, multi, dithering).await?;

    // send the image to discord
    ctx.send(|msg| {
        msg.attachment(AttachmentType::Bytes {
            data: Cow::Borrowed(&art),
            filename: attachment.filename,
        })
    })
    .await?;

    Ok(())
}

/// Generate Minecraft pixel art from your avatar
#[command(slash_command, category = "minecraft")]
pub async fn avatar(
    ctx: Context<'_>,
    #[description = "Whether your server avatar should be used"] server: Option<bool>,
    #[description = "The max height of the pixel art"] height: Option<u16>,
    #[description = "The max width of the pixel art"] width: Option<u16>,
    #[description = "Allow for multiple layers to be generated"] multi: Option<bool>,
    #[description = "Enable dithering"] dithering: Option<bool>,
) -> Result<()> {
    ctx.defer().await?;
    let server = server.unwrap_or(true);
    let author = ctx.author();
    let art = pixel_art(
        ctx.author_member()
            .await
            .map(|x| if server { x.avatar_url() } else { None })
            .flatten()
            .or(author.avatar_url())
            .unwrap_or(author.default_avatar_url()),
        height,
        width,
        multi,
        dithering,
    )
    .await?;

    // send the image to discord
    ctx.send(|msg| {
        msg.attachment(AttachmentType::Bytes {
            data: Cow::Borrowed(&art),
            filename: author.name.clone() + ".png",
        })
    })
    .await?;

    Ok(())
}

#[command(slash_command, subcommands("image", "avatar"), category = "minecraft")]
pub async fn pixel(_ctx: Context<'_>) -> Result<()> {
    Ok(())
}
