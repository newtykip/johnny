use poise::serenity_prelude::{EmojiId, ReactionType};

pub struct Reactions {
    pub upvote: ReactionType,
    pub downvote: ReactionType,
}

impl Default for Reactions {
    fn default() -> Self {
        Self {
            upvote: ReactionType::Custom {
                animated: false,
                id: EmojiId(1120764904656351324),
                name: Some("upvote".into()),
            },
            downvote: ReactionType::Custom {
                animated: false,
                id: EmojiId(1120764921555206336),
                name: Some("downvote".into()),
            },
        }
    }
}
