use std::collections::HashMap;
fn main() {
    let a = HashMap::from([("apple", 1), ("banana", 1)]);
    let b = HashMap::from([("apple", 2), ("peach", 2), ("grape", 2)]);

    let mut total: HashMap<&str, i32> = HashMap::new();

    for (key, value) in a.iter() {
        *total.entry(key).or_insert(0) += value;
    }
    println!("{:#?}", total);
    assert_eq!(total, a);

    for (key, value) in b.iter() {
        *total.entry(key).or_insert(0) += value;
    }
    println!("{:#?}", total);

    let expected_total = HashMap::from([("apple", 3), ("peach", 2), ("grape", 2), ("banana", 1)]);
    assert_eq!(total, expected_total);
}
