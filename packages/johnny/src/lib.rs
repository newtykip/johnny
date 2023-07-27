pub mod events;
mod reactions;

use common::{prelude::*, Data};
use imgurs::ImgurClient;
use poise::serenity_prelude::ChannelId;
use rand::{thread_rng, Rng};
use reactions::Reactions;
use serde::{Deserialize, Serialize};

const GALLERY_IDS: [&str; 2] = ["oPluI3u", "Ca2YQ2O"];
pub const SUGGESTIONS_ID: ChannelId = ChannelId(1120764782014890032);

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// imgur client id
    pub imgur: String,
}

/// Fetch all johnny images from the imgur gallery
pub async fn fetch_images(imgur_token: &str) -> Result<Vec<String>> {
    let client = ImgurClient::new(imgur_token);
    let mut images = vec![];

    // fetch all of the images from each gallery
    for id in GALLERY_IDS {
        images.extend(
            client
                .album_info(id)
                .await?
                .data
                .images
                .par_iter()
                .map(|image| image.link.clone())
                .filter(|link| link.ends_with(".png") || link.ends_with(".jpg"))
                .collect::<Vec<_>>(),
        );
    }

    Ok(images)
}

/// Get a random johnny image
pub fn johnny_image(data: &Data) -> (usize, String) {
    let index = thread_rng().gen_range(0..data.johnny_images.len());

    (index + 1, data.johnny_images[index].clone())
}
