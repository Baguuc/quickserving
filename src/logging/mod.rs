use chrono::{Utc, DateTime};

static FORMATTING: &str = "[%loglevel%] %date% %time% => %message%";

pub enum LogLevel {
    INFO,
    DEBUG,
    ERROR,
    WARN
}

impl Into<String> for LogLevel {
    fn into(self) -> String {
        return match self {
            Self::INFO => "INFO",
            Self::DEBUG => "DEBUG",
            Self::ERROR => "ERROR",
            Self::WARN => "WARN"
        }
        .to_string();
    }
}

pub fn log(level: LogLevel, message: String) {
    let level_str: String = level.into();
    let now: DateTime<Utc> = Utc::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let time_str = now.format("%H:%M:%S").to_string();

    let formatted = FORMATTING
        .replace("%loglevel%", level_str.as_str())
        .replace("%message%", message.as_str())
        .replace("%date%", date_str.as_str())
        .replace("%time%", time_str.as_str());
    
    println!("{}", formatted);
}
