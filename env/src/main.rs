use std::env;
use std::str::FromStr;

fn main() {
    read_parameters_from_env()
}

fn read_parameters_from_env() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(
            u64::from_str(&arg)
                .expect("error parsing argument")
        );
    }
    if numbers.len() < 2 {
        eprintln!("You should enter at least 2 number");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    };
    println!("Result of {:?} is {}", numbers, d)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    return n;
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(8, 16), 8);
}
