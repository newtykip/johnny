pub use super::general::*;
#[cfg(autorole)]
pub use crate::db::AutoroleDB;
pub use crate::db::GetDB;
pub use crate::{embed::*, message_embed, Context};
pub use poise::command;
pub use poise::AutocompleteChoice;
pub use sea_orm::ActiveValue::*;
pub use serenity::model::{
    application::{component::*, interaction::InteractionResponseType},
    prelude::*,
};

pub trait IsEveryone {
    /// check if a role is the @everyone role
    fn is_everyone(&self) -> bool;
}

impl IsEveryone for Role {
    fn is_everyone(&self) -> bool {
        self.id.0 == self.guild_id.0
    }
}
