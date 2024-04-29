extern crate chrono;

use chrono::Local;

use colored::Colorize;

enum LogLevel {
    Info(String),
    Warn(String),
}

pub fn log(msg: &str) {
    print_log(LogLevel::Info(msg.into()));
}

pub fn warn(msg: &str) {
    print_log(LogLevel::Warn(msg.into()));
}

fn print_log(level: LogLevel) {
    let now = Local::now().format("%H:%M:%S");
    match level {
        LogLevel::Info(s) => println!("{} -- {}", now, s),
        LogLevel::Warn(s) => println!("--- {} ---", s.red()),
    }
}
