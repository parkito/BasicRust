#![allow(dead_code)]
#![allow(unused_variables)]

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
    assert_eq!(std::char::from_digit(2,10), Some(2));
}

fn overflow() {
    //in release i will be negative during overflow
    let mut i: i32 = 1;
    loop {
        // i *= 10;
        i = i.checked_mul(10).expect("overflow") //how you can trigger panic in release build
    }
}
