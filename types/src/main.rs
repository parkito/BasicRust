#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::format;
use std::ops::Add;

fn main() {
    //basic types
    let i1: i32 = 10;
    let u1: u32 = 10; // unsigned type
    let i2: isize = 10; //address dependant int
    let f1: f32 = 1.0;
    let b1: bool = false;
    let ch1: char = 'c';
    let tup1: (i32, i32) = (1, 1);
    let tup2: (i32, i32, char) = (1, 1, 'b');
    let str1: String = "one".to_string().add("a");
    let str2: &str = "two";
    let vec1: Vec<i32> = vec![1, 2, 3];
    let opt1: Option<i32> = Some(5);
    let opt2: Option<i32> = None;

    assert_eq!(2_u16.pow(2), 4_u16);
    assert_eq!(0b1011_u8.count_ones(), 3);
    assert_eq!(1000_i16 as u8, 232_u8);
    assert_eq!((4_i32.abs()), 4_i32); //wtf with -4
    assert_eq!(i32::abs(-4), 4);

    // overflow();

    //checked operations
    assert_eq!(10_i32.checked_mul(1000), Some(10_000_i32));
    assert_eq!(100_u8.checked_mul(200), None); //overflow
    // assert_eq!(100_u8.checked_mul(200).unwrap(), 100); //overflow with panic

    //wrapped operations return the value equivalent to the mathematically correct result module the rande of a value
    assert_eq!(500_u16.wrapping_mul(500), 53392_u16); //u16 overflow, so return u16 max instead

    //saturated operations - providing the closed mathematically correct value
    assert_eq!((-32760_i16).saturating_sub(10), -32768); //u16 overflow, so return u16 max instead

    //overflowing operations return a tuple with a result and status of overflowing
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));

    //floats
    assert_eq!(-f32::MIN, f32::MAX);
    assert_eq!(1. / 0., f32::INFINITY);
    assert_eq!(-1. / 0., -f32::INFINITY);

    //characters
    assert_eq!('*'.is_alphabetic(), false);
    assert_eq!('1'.is_digit(2), true);
    assert_eq!(std::char::from_digit(2, 10), Some('2'));

    //unit type
    let uni: () = {};

    //references
    let i: i32 = 5;
    let pi1: &i32 = &i;
    // let mut pi2: &i32 = &i;

    //boxes
    let t1 = (12, "eggs");
    let t1b: Box<(i32, &str)> = Box::new(t1); //goes to heap

    //arrays
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4];
    println!("{}", arr2.len());

    let mut arr3 = [0; 10]; // arrays of size 10 with only zeros
    arr3[1] = 1;

    let vec1: Vec<i32> = Vec::new();
    let vec2: Vec<i32> = vec![1, 23];
    let vec3: Vec<i32> = vec![1; 5]; //vector of size 5 filled with 1
    let vec4: Vec<i32> = (0..10).collect(); //vector from iterator
    let mut vec5: Vec<i32> = Vec::with_capacity(10); //capacity 10, len 2
    // println!("{}", vec5[0]); //panic
    vec5.push(1);
    println!("{}", vec5[0]);

    //slices
    let vec6: Vec<i32> = vec![1, 2, 3, 4];
    let arr3: [i32; 4] = [1, 2, 3, 4];
    let sv1: &[i32] = &vec6; //slice of array
    let sv2: &[i32] = &arr3; //slice of vector
    println!("{}", sv1.len());

    print_slice(sv1);
    print_slice(sv2);
    print_slice(&sv1[0..2]); //part of the slice

    //strings
    let literal = "string";
    let raw1 = r"C:\program Files\ ";
    let raw2 = r###"
    This
    is string with stuff like " or '
    "###;

    let byte_str = b"String";
    let row_byte_str = br"\\//String";
    //&str is a fat pinter to u8 vector

    assert_eq!("©©".len(), 4);//in bytes
    assert_eq!("©©".chars().count(), 2);//number of symbols

    let str1: String = "string".to_string();
    let str2: String = format!("THis is {}", 2).to_string();
    let vec7 = vec!["on1", "two"];
    let concated = vec7.concat();
    let joined = vec7.join(",");

    //Aliases
    type NewType = Vec<Vec<i32>>;
    let tp: NewType = vec![vec![1, 2, 3]];
}

fn print_slice(slice: &[i32]) {
    for sl in slice {
        print!("{} ", sl)
    };
    println!()
}

fn overflow() {
    //in release i will be negative during overflow
    let mut i: i32 = 1;
    loop {
        // i *= 10;
        i = i.checked_mul(10).expect("overflow") //how you can trigger panic in release build
    }
}
