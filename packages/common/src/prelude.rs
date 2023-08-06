pub use color_eyre::{
    eyre::{eyre, Context as EyreContext, ContextCompat, Error, Result},
    Help,
};
pub use rayon::prelude::*;
pub use serenity::model::prelude::*;

pub trait IsEveryone {
    /// check if a role is the @everyone role
    fn is_everyone(&self) -> bool;
}

impl IsEveryone for Role {
    fn is_everyone(&self) -> bool {
        self.id.0 == self.guild_id.0
    }
}
