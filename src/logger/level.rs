#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Command,
    Event,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Command => "COMMAND".to_string(),
            LogLevel::Event => "EVENT".to_string(),
        }
    }
}
