#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;

struct Person {
    name: String,
    second_name: Option<String>,
}

#[derive(Copy, Clone)]
struct CopyablePerson {
    name: i32,
    second_name: Option<i32>,
}

fn main() {
    let mut vec: Vec<i32> = Vec::new();
    for i in 0..10 {
        vec.push(i);
    }
    let last = vec.pop();
    //changing ownership
    //by removing
    let second = vec.swap_remove(1);
    //by replacing
    let third = std::mem::replace(&mut vec[3], 100);


    let mut persons = vec![Person {
        name: "name".to_string(),
        second_name: Some("second".to_string()),
    }];
    // can't do that because you change the ownership
    // let f_name = persons[0].name;
    // let s_name = persons[0].second_name;
    let borrowed1 = std::mem::replace(&mut persons[0].second_name, None);
    let borrowed2 = persons[0].second_name.take();
    assert_eq!(borrowed2, None);

    let new_person = Person {
        name: "name".to_string(),
        second_name: Some("second".to_string()),
    };
    print_person(new_person);
    // println!("{}", new_person.name); can't do that. print_person took the ownership

    //all fields of user-defined Copyable types must be copyable
    let new_copyable_person = CopyablePerson {
        name: 1,
        second_name: Some(2),
    };
    print_copyable_person(new_copyable_person);
    println!("{}", new_copyable_person.name); //can do that. print_copyable_person received a copy of an object

    //Arc uses atomic references, so its thread sage
    //Rc is not thread safe but faster

    let rc1: Rc<String> = Rc::new("hello".to_string());
    let rc2 = rc1.clone(); //no copy, just internal counter increment
    // rc1.push("f".to_string()) Rc is immutable
}

fn print_person(person: Person) {
    println!("{}", person.name)
}

fn print_copyable_person(person: CopyablePerson) {
    println!("{}", person.name)
}


