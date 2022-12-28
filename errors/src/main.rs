use std::error::Error;
use std::io::stderr;

fn main() {
    println!("Hello, world!");
}

fn print_error(mut err: &dyn Error) {
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

fn read_file() -> Result<i32, std::io::Error> {
    let res = read_file_lower()?; // propagate in case of error further above
    //res is unwrapped i32 now
}

fn read_file_lower() -> Result<i32, std::io::Error> {
    return Ok(1);
}