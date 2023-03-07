#![allow(dead_code)]
#![allow(unused_variables)]

static mut VARIABLE: &i32 = &100;

//live till the end of programm
struct S{
    r: &'static i32
}

struct Sa<'a>{
    r: &'a i32
}

fn main() {
    let mut vec: Vec<i32> = vec![1, 2, 3, 4];
    show_vec(&mut vec);

    let var_a = 5;
    let mut var_b = 10;
    var_b /= 2;

    let ref1 = &var_a;
    let mut ref2 = &var_b;

    assert_eq!(ref1, ref2); //assert value
    assert!(!std::ptr::eq(ref1, ref2)); //assert references

    //cannot use mutable static in several threads
    // show_var_1(VARIABLE);
    // show_var_2(VARIABLE);
}

fn show_var_1(var: &i32) {
    println!("{}", var)
}

fn show_var_2<'a>(var: &'static i32) {
    println!("{}", var);
}

fn show_vec(vec: &mut Vec<i32>) {
    for x in vec {
        print!("{} ", x)
    }
}
