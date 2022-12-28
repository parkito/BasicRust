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

// struct Log {}
//
// impl Log {
//     fn log(level: Level, msg: &str) {
//         println!("{}, {:?}", level, msg);
//     }
// }


fn main() {
    println!("Hello, world!");
}

#[test]
fn levels_have_correct_names() {
    for level in Level::VALUES {
        assert_eq!(level.0.to_str(), level.1);
    };
}