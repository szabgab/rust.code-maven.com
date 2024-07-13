use std::collections::HashMap;
fn main() {
    let a = HashMap::from([
        ("apple", 1),
        ("banana", 1),
    ]);
    let b = HashMap::from([
        ("apple", 2),
        ("peach", 2),
        ("grape", 2),
    ]);

    let mut total: HashMap<&str, i32> = HashMap::new();
    total.extend(a);
    println!("{:#?}", total);

    total.extend(b);
    println!("{:#?}", total);
}
