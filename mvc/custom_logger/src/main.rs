#![allow(dead_code)]
#![allow(unused_variables)]

extern crate core;

use std::fmt;
use std::fmt::{Formatter};
use crate::Level::{DEBUG, FATAL, INFO, NONE, TRACE, WARN};
use crate::LogType::STD;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    NONE,
    FATAL,
    WARN,
    INFO,
    DEBUG,
    TRACE,
}

impl Level {
    const VALUES: [(Self, &'static str); 6] = [
        (NONE, "NONE"),
        (FATAL, "FATAL"),
        (WARN, "WARN"),
        (INFO, "INFO"),
        (DEBUG, "DEBUG"),
        (TRACE, "TRACE")
    ];

    pub fn to_str(&self) -> &'static str {
        return Level::VALUES[*self as usize].1;
    }
}

impl fmt::Display for Level {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        fmt.pad(self.to_str())
    }
}

enum LogType {
    FILE,
    #[allow(non_camel_case_types)]
    ENV_STREAMS,
    STD,
}

struct LogSetting {
    log_type: LogType,
    file_path: Option<String>,
    root_level: Option<Level>,
}

struct LogFactory {}

impl LogFactory {
    fn log_setting() -> &'static Option<LogSetting> {
        static LOG_SETTING: &Option<LogSetting> = &None;
        return LOG_SETTING;
    }

    fn set_log_setting(setting: LogSetting) {
        match LogFactory::log_setting() {
            None => LogFactory::set_log_setting(setting),
            Some(se) => panic!("Logger setting is already defined")
        }
    }

    fn init() {
        match LogFactory::log_setting() {
            None => LogFactory::set_log_setting(LogSetting { log_type: STD, file_path: None, root_level: None }),
            _ => {}
        }
    }

    fn build(logger: &str) -> Box<dyn Logger> {
        match LogFactory::log_setting() {
            None => LogFactory::init(),
            Some(_) => {}
        }

        let settings = LogFactory::log_setting();
        return match settings {
            None => panic!("Settings at this stage must exist"),
            Some(setting) => Box::new(StdLogger { name: logger.to_string() })
        };
    }
}

trait Logger {
    fn name(&self) -> String;

    fn log(&self, level: Level, msg: &str);

    fn format(&self, level: Level, logger_name: &str, msg: &str) -> String {
        let now = chrono::offset::Local::now().to_string();
        return format!("{} [{}]: {}", now, logger_name, msg);
    }
}

struct StdLogger {
    name: String,
}

impl Logger for StdLogger {
    fn name(&self) -> String {
        return self.name.to_string();
    }

    fn log(&self, level: Level, msg: &str) {
        // self.format(level)
        println!("hello")
    }
}

#[test]
fn levels_have_correct_names() {
    for level in Level::VALUES {
        assert_eq!(level.0.to_str(), level.1);
    };
}

#[test]
fn print_logs_in_std() {
    let logger = LogFactory::build("first_logger");
    logger.log(Level::INFO, "first message");
}