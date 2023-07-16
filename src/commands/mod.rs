mod general;
mod image;
mod moderation;

#[cfg(autorole)]
pub use moderation::autorole;

#[cfg(pride)]
pub use self::image::pride;

pub use general::ping;
