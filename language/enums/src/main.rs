use std::any::Any;
use std::fmt::Debug;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
enum ComplexEnum {
    ONE,
    TWO,
    THREE(i32),
    LINE(String),
    FOUR(i32),
}

struct SomeStruct {
    line: String,
    number: i32,
}

fn print_direction(dir: Direction) {
    match dir {
        Direction::Up => println!("To up"),
        Direction::Down => println!("To down"),
        Direction::Left => println!("To left"),
        Direction::Right => println!("To right"),
    }
}

fn main() {
    let a = ComplexEnum::LINE("SOMETHING".to_string());
    match a {
        ComplexEnum::THREE(3) => println!("Found 3"),
        ComplexEnum::LINE(some) => println!("Found {}", some),
        _el => () //nothing to return
    }

    let b = SomeStruct { line: "line".to_owned(), number: 42 };
    match b {
        SomeStruct { number: 50, line } => println!("Numer 50 and line is{}", line),
        SomeStruct { number: 40, .. } => println!("Numer 40 and line is"),
        SomeStruct { line, .. } => println!("{}", line), //other arguments other then line are unimportant
    }
}
