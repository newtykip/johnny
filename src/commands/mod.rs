#[cfg(autorole)]
mod autorole;
mod general;
#[cfg(image)]
mod image;

#[cfg(autorole)]
pub use autorole::autorole;

#[cfg(image)]
pub use self::image::pride;

pub use general::ping;
