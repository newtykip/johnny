use entity::{guild, user};
use poise::serenity_prelude::{GuildId, UserId};
use sea_orm::ActiveValue::*;

pub fn create_user(id: UserId) -> user::ActiveModel {
    user::ActiveModel {
        id: Set(id.to_string()),
    }
}

pub fn create_guild(id: GuildId) -> guild::ActiveModel {
    guild::ActiveModel {
        id: Set(id.to_string()),
        ..Default::default()
    }
}
