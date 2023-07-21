#[cfg(not(tui))]
use owo_colors::{OwoColorize, Style as OwoStyle};
#[cfg(tui)]
use ratatui::style::{Color as RatColour, Modifier as RatModifier, Style as RatStyle};

#[derive(Debug, Clone, Copy)]
pub enum Colour {
    Red,
    Cyan,
}

#[derive(Debug, Clone, Copy)]
pub struct Style {
    colour: Option<Colour>,
    bold: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            colour: None,
            bold: false,
        }
    }
}

impl Style {
    pub fn colour(mut self, colour: Colour) -> Self {
        self.colour = Some(colour);
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
}

#[cfg(not(tui))]
impl Into<OwoStyle> for Style {
    fn into(self) -> OwoStyle {
        let mut style = OwoStyle::default();

        // apply colour
        if let Some(colour) = self.colour {
            match colour {
                Colour::Red => style = style.red(),
                Colour::Cyan => style = style.cyan(),
            }
        }

        // apply bold
        if self.bold {
            style = style.bold();
        }

        style
    }
}

#[cfg(tui)]
impl Into<RatStyle> for Style {
    fn into(self) -> RatStyle {
        let mut style = RatStyle::default();

        // apply colour
        if let Some(colour) = self.colour {
            style = style.fg(match colour {
                Colour::Red => RatColour::Red,
                Colour::Cyan => RatColour::Cyan,
            });
        }

        // apply bold
        if self.bold {
            style = style.add_modifier(RatModifier::BOLD);
        }

        style
    }
}
