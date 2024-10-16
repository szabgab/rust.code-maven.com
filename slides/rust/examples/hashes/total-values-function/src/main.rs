use std::collections::HashMap;
fn main() {
    let a = HashMap::from([("apple", 1), ("banana", 1)]);
    let b = HashMap::from([("apple", 2), ("peach", 2), ("grape", 2)]);

    let mut total: HashMap<&str, i32> = HashMap::new();

    add(&mut total, &a);
    println!("{:#?}", total);

    add(&mut total, &b);
    println!("{:#?}", total);
}

fn add<'a>(total: &mut HashMap<&'a str, i32>, other: &HashMap<&'a str, i32>) {
    for (key, value) in other.iter() {
        *total.entry(key).or_insert(0) += value;
    }
}

// fn add(total: &mut HashMap<&str, i32>, other : &HashMap<&str, i32>) {
//     for (key, value) in other.iter() {
//         *total.entry(key).or_insert(0) += value;
//     }
// }
