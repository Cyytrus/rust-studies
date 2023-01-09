
// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
fn main() {
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    return format!("[INFO]: {}", message);
}
pub fn warn(message: &str) -> String {
    return format!("[WARNING]: {}", message);
}
pub fn error(message: &str) -> String {
    return format!("[ERROR]: {}", message);
}
