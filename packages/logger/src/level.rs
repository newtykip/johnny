pub enum LogLevel {
    Info,
    Warn,
    Error,
    #[cfg(verbose)]
    Command,
}

impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Info => "INFO".to_string(),
            LogLevel::Warn => "WARN".to_string(),
            LogLevel::Error => "ERROR".to_string(),
            #[cfg(verbose)]
            LogLevel::Command => "COMMAND".to_string(),
        }
    }
}
