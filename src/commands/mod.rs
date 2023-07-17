// ! general
mod general;

pub use general::ping;

// ! moderation
mod moderation;

#[cfg(autorole)]
pub use moderation::autorole;

// ! image
mod image;

#[cfg(pride)]
pub use self::image::pride;
