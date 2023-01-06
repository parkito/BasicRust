#[derive(Default, Debug, PartialEq)]
struct MyConf {
    int: i32,
    double: f64,
    boolean: bool,
    str: String,
    vec: Vec<i32>,
}

impl MyConf {}


fn main() {
    let conf = MyConf {
        vec: vec![1, 2], //explicit setting vector
        ..Default::default() // other members are default
    };
    println!(
        "int default {}\ndouble default {}\nbool default {}\nstr default {}\nvec default {}",
        conf.int, conf.double, conf.boolean, conf.str, conf.vec[0]
    );
}
