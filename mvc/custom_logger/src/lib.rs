mod appenders;

use std::fmt;
use std::fmt::{Formatter};
use std::sync::Mutex;
use custom_file_io::{FileAppender, FileIoFactory};
use LogType::FILE;
use crate::appenders::StdLogger;
use crate::appenders::FileLogger;
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
    pub const VALUES: [(Self, &'static str); 6] = [
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

#[derive(Debug, Clone)]
pub enum LogType {
    FILE,
    #[allow(non_camel_case_types)]
    ENV_STREAMS,
    STD,
}

#[derive(Debug, Clone)]
pub struct LogSetting {
    pub log_type: LogType,
    pub file_path: Option<String>,
    pub root_level: Option<Level>,
}

struct LogContext {
    appender: FileAppender,
}

static LOG_SETTING: Mutex<Option<LogSetting>> = Mutex::new(None);
// static LOG_CONTEXT: Mutex<Option<HashMap<String, LogContext>>> = Mutex::new(None);

pub struct LogFactory {}

impl LogFactory {
    pub fn log_setting() -> Option<LogSetting> {
        return match LOG_SETTING.lock().unwrap().as_ref() {
            None => None,
            Some(s) => Some(s.clone())
        };
    }

    pub fn set_log_setting(setting: LogSetting) {
        LOG_SETTING.lock().unwrap().replace(setting);
    }

    fn init() {
        match LogFactory::log_setting() {
            None => LogFactory::set_log_setting(LogSetting { log_type: STD, file_path: None, root_level: None }),
            _ => {}
        }
    }

    pub fn build(logger: &str) -> Box<dyn Logger> {
        match LogFactory::log_setting() {
            None => LogFactory::init(),
            Some(_) => {}
        }

        let settings = LogFactory::log_setting();
        return match settings {
            None => panic!("Settings at this stage must exist"),
            Some(_setting) => {
                match _setting.log_type {
                    FILE => {
                        Box::new(
                            FileLogger {
                                name: logger.to_string(),
                                appender: FileAppender {
                                    writer: Mutex::new(FileIoFactory::create_buf_writer(&_setting.file_path.expect("")))
                                },
                            }
                        )
                    }
                    _ => Box::new(StdLogger { name: logger.to_string() })
                }
            }
        };
    }
}

pub trait Logger {
    fn name(&self) -> String;

    fn log(&self, level: Level, msg: &str);

    fn format(&self, level: Level, logger_name: &str, msg: &str) -> String {
        let now = chrono::offset::Local::now().to_string();
        return format!("{} {} [{}]: {}", now, level.to_str(), logger_name, msg);
    }
}