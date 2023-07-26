pub use crate::components;
pub use crate::logger::{methods as logger, Style as LogStyle};
pub use cfg_if::cfg_if;
pub use color_eyre::{
    eyre::{eyre, Context as EyreContext, ContextCompat, Error, Result},
    Help,
};
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
