use std::collections::HashMap;

fn main() {
    let mut pairs: Vec<(String, i32)> = vec![(String::from("snake"), 1), (String::from("dog"), 2)];
    let cat = (String::from("cat"), 3);
    pairs.push(cat);
    println!("vector of pairs: {:?}", pairs);

    let counter: HashMap<String, i32> = HashMap::from_iter(pairs);
    println!("hash: {:?}", counter);
    println!("dog: {:?}", counter["dog"]);
}
