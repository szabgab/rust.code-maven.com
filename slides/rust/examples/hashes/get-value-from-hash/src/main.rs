use std::collections::HashMap;

fn main() {
    let counter = HashMap::from([
        ("foo", 1),
        ("bar", 2),
    ]);
    println!("{}", counter["foo"]);
    println!("{:?}", counter.get("foo"));

    // println!("{}", counter["zz"]); // panic
    println!("{:?}", counter.get("zz")); // None

    println!();

    match counter.get("foo") {
        Some(val) => println!("{val}"),
        None => println!("None"),
    };

    match counter.get("zz") {
        Some(val) => println!("{val}"),
        None => println!("None"),
    };

}
