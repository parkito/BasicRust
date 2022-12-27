#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    contains_a(&"Hello".to_string());
    contains_a(&"HOAa".to_string());

    if let Ok(element) = "100".parse() {
        // True if 50, 100 or 150.
        let result = matches!(element, 50 | 100 | 150);
        println!("{}", result);
    }

    let some = Some("ablaf");
    if let Some(name) = some {
        println!("{}", some.unwrap())
    }

    let vec = ret_vec();

    let a1 = &vec[0..1]; //start, end
    let a2 = &vec[..]; //full range
    let a3 = &vec[..1]; //0 , end
    let a4 = &vec[1..]; //start, size-1
    let a5 = &vec[1..=3]; //start, end inclusive

    print_slice(a1);
    print_slice(a2);
    print_slice(a3);
    print_slice(a4);
    print_slice(a5);

    //closures
    let is_even = |x: i32| -> bool { x % 2 == 0 };
    let is_even1 = |x: i32| x % 2 == 0;

    println!("{}", is_even(1));
    println!("{}", is_even1(1));
}

fn print_slice(slice: &[i32]) {
    for x in slice {
        print!("{} ", x);
    }
    println!();
}

fn divergent() -> ! {
    loop {
        println!("Infinite loop")
    }
}

fn contains_a(str: &String) -> bool {
    return match str.find("a") {
        Some(position) => {
            println!("{}", position);
            true
        }
        _none =>
            {
                println!("Not found");
                false
            }
    };
}

fn ret_vec() -> Vec<i32> {
    let mut vec = Vec::<i32>::with_capacity(10);
    for i in 0..10 {
        vec.push(i);
    }
    return vec;
}