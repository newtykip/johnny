use super::LogLevel;
use chrono::{DateTime, Local};
#[cfg(not(tui))]
use owo_colors::{
    colors::{BrightWhite, Cyan, Green, Yellow},
    OwoColorize,
    Stream::Stdout,
};
#[cfg(tui)]
use poise::serenity_prelude::{ChannelId, Guild, User};

#[derive(Debug, Clone)]
pub struct Entry {
    pub timestamp: DateTime<Local>,
    pub level: LogLevel,
    pub message: String,
    #[cfg(tui)]
    pub guild: Option<Guild>,
    #[cfg(tui)]
    pub user: Option<User>,
    #[cfg(tui)]
    pub channel: Option<ChannelId>,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl ToString for Entry {
    fn to_string(&self) -> String {
        let timestamp = self.timestamp.format("%Y-%m-%d %H:%M:%S");
        let level = format!("[{}]", self.level.to_string());

        #[cfg(not(tui))]
        let timestamp = timestamp.if_supports_color(Stdout, |text| text.fg::<Cyan>());
            
        #[cfg(not(tui))]
        let level = level.if_supports_color(Stdout, |text| match self.level {
            LogLevel::Info => text.fg::<BrightWhite>().bold().to_string(),
            LogLevel::Command => text.fg::<Green>().bold().to_string(),
            LogLevel::Warn => text.fg::<Yellow>().bold().to_string(),
        });

        format!("{} {} {}", timestamp, level, self.message)
    }
}
