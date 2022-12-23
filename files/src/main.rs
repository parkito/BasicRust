use std::fs::File;
use std::io::prelude::*;
use std::{fs, str};

fn main() {
    print_file();
    write_hello();
}

fn print_file() {
    let row_content = read_file("/tmp/1.txt")
        .expect("Cannot read the file");
    let lines = str::from_utf8(&row_content).unwrap();
    println!("{}", lines);
}

fn write_hello() {
    write_to_file("/tmp/2.txt", "Hello");
}

fn write_to_file(path: &str, content: &str) {
    match fs::write(path, content) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Cannot write to {} with error {:?}", path, e);
            std::process::exit(1);
        }
    }
}

fn read_file(path: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut vec = Vec::new();
    let _ = file.read_to_end(&mut vec);
    return Ok(vec);
}