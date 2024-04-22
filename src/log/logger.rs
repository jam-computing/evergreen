use colored::Colorize;

enum LogLevel {
    Info(String),
    Warn(String)
}

pub fn log(msg: &str) {
    print_log(LogLevel::Info(msg.into()));
}

pub fn warn(msg: &str) {
    print_log(LogLevel::Info(msg.into()));
}

fn print_log(level: LogLevel) {
    match level {
        LogLevel::Info(s) => println!("--- {} ---", s),
        LogLevel::Warn(s) => println!("--- {} ---", s.red()),
    }
}
