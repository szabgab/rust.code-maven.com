use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Person {
    fname: String,
    lname: String,
}

fn main() {
    let a = Person {
        fname: String::from("Steve"),
        lname: String::from("Jobs"),
    };

    let m = Person {
        fname: String::from("Bill"),
        lname: String::from("Gates"),
    };

    let mut lookup = HashMap::new();
    lookup.insert(a, "Apple");
    lookup.insert(m, "Microsoft");

    println!("{:?}", lookup);
}
