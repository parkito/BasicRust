use crate::SomeEnum::ONE;
use chrono::{Datelike, DateTime, Timelike, Utc};

#[derive(Debug, Copy, Clone)]
enum SomeEnum {
    ONE,
    TWO,
    THREE,
}

#[derive(Debug, Clone)]
struct Worker {
    name: String,
    date: DateTime<Utc>,
}


fn main() {
    println!("{:?}", ONE);
    let now = Utc::now();

    println!("{:?}", now);
}
