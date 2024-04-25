use std::collections::HashMap;

fn main() {
    let animal = HashMap::from([
        ("snake", "long"),
        ("giraffe", "tall"),
        ("elephant", "heavy"),
    ]);
    println!("{:#?}", animal);

    println!("{:#?}", animal.keys());

    println!("{:#?}", animal.values());

    // animal.insert("cat", "cute"); // cannot borrow `animal` as mutable, as it is not declared as mutable
}
