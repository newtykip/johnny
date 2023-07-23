pub enum LogLevel {
    Info,
    Warn,
    Error,
    Command,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warn => "WARN".to_string(),
            LogLevel::Error => "ERROR".to_string(),
            LogLevel::Command => "COMMAND".to_string(),
        }
    }
}
