use std::collections::HashMap;

fn main() {
    let mut lookup = HashMap::new();
    let a = ("Foo", 2);
    lookup.insert(a, 4);

    lookup.insert(("Bar", 4), 6);

    // The key must be of the same type
    // lookup.insert(("Other", 5, 6), 7);

    println!("{:?}", lookup);
}
