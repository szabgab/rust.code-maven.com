use std::collections::HashMap;


fn main() {
    let mut lookup = HashMap::new();
    lookup.insert(0, "Sunday");
    lookup.insert(1, "Monday");
    lookup.insert(2, "Tuesday");

    println!("{:?}", lookup);
}
