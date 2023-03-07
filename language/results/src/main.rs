#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;
use std::fmt::{Display, Formatter};
use crate::Status::{KNOWN, UNKNOWN};

fn main() {
    let res3 = ok_or_error_intermediate("know").unwrap();
    let res1 = ok_or_error("know").unwrap();
    let res2 = nothing_or_error("dont know").unwrap();
}

// #[derive(Display, Debug, Copy, Clone)]
enum Status {
    KNOWN,
    UNKNOWN,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            KNOWN => "known",
            UNKNOWN => "unknown",
        };
        write!(f, "{}", printable)
    }
}

impl Status {}

fn ok_or_error(status: &str) -> Result<Status, String> {
    if status == "know" {
        return Ok(KNOWN);
    }
    return Err("Something is wrong".to_string());
}

fn ok_or_error_intermediate(status: &str) -> Result<Status, String> {
    let ok = ok_or_error(status)?;
    println!("You see that because I get ok {}", ok);
    Ok(KNOWN)
}

fn nothing_or_error(status: &str) -> Result<(), String> {
    if status == "know" {
        return Ok(());
    }
    return Err("Something is wrong".to_string());
}