use std::collections::HashMap;


#[derive(Debug)]
#[allow(dead_code)]
struct Something {
    key1: String,
    key2: String,
}

fn main() {
    let a = Something { key1: String::from("one"), key2: String::from("two") };
    let b = Something { key1: String::from("uno"), key2: String::from("dos") };
    dbg!(&a);
    dbg!(&b);

    let mut mapping_from_key1: HashMap<String, &Something> = HashMap::new();
    mapping_from_key1.insert(a.key1.clone(), &a);
    mapping_from_key1.insert(b.key1.clone(), &b);

    let mut mapping_from_key2: HashMap<String, &Something> = HashMap::new();
    mapping_from_key2.insert(a.key2.clone(), &a);
    mapping_from_key2.insert(b.key2.clone(), &b);

    dbg!(&mapping_from_key1);
    dbg!(&mapping_from_key2);
}
