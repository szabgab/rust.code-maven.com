use std::collections::HashMap;


#[derive(Debug)]
#[allow(dead_code)]
struct Something {
    key1: String,
    key2: String,
}

fn main() {
    let things: Vec<Something> = vec![
        Something { key1: String::from("one"), key2: String::from("two") },
        Something { key1: String::from("uno"), key2: String::from("dos") },
    ];
    dbg!(&things);

    let mut mapping_from_key1: HashMap<String, &Something> = HashMap::new();
    let mut mapping_from_key2: HashMap<String, &Something> = HashMap::new();
    for thing in &things {
        mapping_from_key1.insert(thing.key1.clone(), thing);
        mapping_from_key2.insert(thing.key2.clone(), thing);
    }

    dbg!(&mapping_from_key1);
    dbg!(&mapping_from_key2);
}
