//Rust does not have constructors as a language construct.
// Instead, the convention is to use an associated function new to create an object:

// #[derive(Default)] //can be derived as well
struct SomeStruct {
    data: String,
}

impl SomeStruct {
    pub fn new(data: String) -> Self {
        Self { data }
    }

    pub fn data_val(&self) -> String {
        return self.data.to_string();
    }
}

impl Default for SomeStruct {
    fn default() -> Self {
        Self { data: "default".to_string() }
    }
}

fn main() {
    let str = SomeStruct::new("SomeData".to_string());
    let def = SomeStruct::default();
    println!("{}", str.data_val());
    println!("{}", def.data_val());
}

// Note: It is common and expected for types to implement both Default and an empty new constructor.
// new is the constructor convention in Rust, and users expect it to exist,
// so if it is reasonable for the basic constructor to take no arguments,
// then it should, even if it is functionally identical to default.

// Hint: The advantage of implementing or deriving Default is that your type can now be used where
// a Default implementation is required, most prominently, any of the *or_default functions in the standard library.
