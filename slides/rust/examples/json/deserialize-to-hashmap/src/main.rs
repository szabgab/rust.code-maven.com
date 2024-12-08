use std::collections::HashMap;

fn main() {
    let filename = "data.json";
    let content = std::fs::read_to_string(filename).unwrap();

    let leggs: HashMap<String, u32> = serde_json::from_str(&content).unwrap();
    println!("leggs:     {leggs:?}");

    // doing the same using Turbofish
    let turbofish = serde_json::from_str::<HashMap<String, u32>>(&content).unwrap();
    println!("turbofish: {turbofish:?}");

    assert_eq!(leggs, turbofish);
}
