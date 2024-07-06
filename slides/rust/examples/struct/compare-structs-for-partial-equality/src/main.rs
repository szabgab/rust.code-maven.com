#[derive(PartialEq)]
struct Thing {
    name: String,
    number: i32,
    float: f32,
}

fn main() {
    let a = Thing {
        name: String::from("Foo"),
        number: 42,
        float: 1.0,
    };

    let b = Thing {
        name: String::from("Foo"),
        number: 42,
        float: 1.0,
    };

    let c = Thing {
        name: String::from("Foo1"),
        number: 42,
        float: 1.0,
    };

    println!("{}", a == b);
    println!("{}", a == c);

    // must implement `PartialOrd`
    // println!("{}", a < c);
}
