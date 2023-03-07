fn main() {
    let tuple = one_two_three();
    let (a, b, c) = one_two_three();

    println!("t1 is {}, t2 is {}, t3 is {}", tuple.0, tuple.1, tuple.2);
    println!("t1 is {}, t2 is {}, t3 is {}", a, b, c);
}

fn one_two_three() -> (i32, char, isize) {
    (1, '2', 3)
}
