#![allow(dead_code)]
#![allow(unused_variables)]

extern crate core;

use custom_file_io::FileAppender;
use crate::{Level, Logger};

pub struct StdLogger {
    pub name: String,
}

impl Logger for StdLogger {
    fn name(&self) -> String {
        return self.name.to_string();
    }

    fn log(&mut self, level: Level, msg: &str) {
        let line = self.format(level, &*self.name, msg);
        println!("{}", line)
    }
}

pub struct FileLogger {
    pub name: String,
    pub appender: FileAppender,
}

impl Logger for FileLogger {
    fn log(&mut self, level: Level, msg: &str) {
        let line = self.format(level, &*self.name, msg);
        self.appender.append(line.as_str());
    }

    fn name(&self) -> String {
        return self.name.to_string();
    }
}

