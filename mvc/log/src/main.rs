extern crate core;

use std::fmt;
use std::fmt::{Formatter};
use crate::Level::{DEBUG, FATAL, INFO, NONE, TRACE, WARN};

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
    fn build(setting: LogSetting) -> Logger {
        return Logger { setting };
    }
}

struct Logger {
    setting: LogSetting,
}

struct Log {}

impl Log {
    fn log_setting() -> &'static Option<LogSetting> {
        static LOG_SETTING: &Option<LogSetting> = &None;
        return LOG_SETTING;
    }

    fn set_log_setting(setting: LogSetting) {
        if Log::log_setting() == None {
            Log::set_log_setting(setting);
        } else {
            panic!("Logger setting is already defined");
        }
    }

    fn logger() -> &'static Logger {
        static LG: &Logger = &LogFactory::build(LogSetting("".to_string(), TRACE));
        return LG;
    }
    fn log(level: Level, msg: String) {}
}

#[test]
fn levels_have_correct_names() {
    for level in Level::VALUES {
        assert_eq!(level.0.to_str(), level.1);
    };
}

#[test]
fn print_logs_in_std() {
    let setting = LogSetting { log_type: LogType::STD, file_path: None, root_level: None };
    Log::set_log_setting(setting);
    Log::log(Level::INFO, "hello".to_string());
}
