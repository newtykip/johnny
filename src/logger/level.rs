#[derive(Debug, Clone, PartialEq)]
pub enum LogLevel {
    Info,
    Command,
    Warn,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Command => "COMMAND".to_string(),
            LogLevel::Warn => "WARN".to_string(),
        }
    }
}
