#![allow(dead_code)]
#![allow(unused_variables)]

extern crate core;

use crate::{Level, Logger};

pub struct StdLogger {
    pub name: String,
}

impl Logger for StdLogger {
    fn name(&self) -> String {
        return self.name.to_string();
    }

    fn log(&self, level: Level, msg: &str) {
        let line = self.format(level, &*self.name, msg);
        println!("{}", line)
    }
}

struct FileLogger {}
