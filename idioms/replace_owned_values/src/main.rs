use std::mem;


fn main() {
    let mye = &mut MyEnum::A { name: "this".to_string(), x: 1 };
    a_to_b(mye);
    match mye {
        MyEnum::A { name, x } => {
            println!("This is A {} {}", name, x)
        }
        MyEnum::B { name } => {
            println!("This is B {}", name)
        }
    }
}

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 1 } = e {
        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*e`).

        // *e = MyEnum::B { name: mem::take(name) } //replacing with default value
        *e = MyEnum::B { name: mem::replace(&mut "swapped".to_string(), name.to_string()) }
    }
}
