use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    email: String,
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <json_file>", args[0]);
        std::process::exit(1);
    }
    let json_file = &args[1];
    let content = std::fs::read_to_string(json_file).expect("Unable to read file");
    let person = serde_json::from_str::<Person>(&content).expect("Unable to parse JSON");
    println!("{:?}", person);
    println!("-------");


    let json_deserializer = &mut serde_json::Deserializer::from_str(&content);
    let mut unused = HashSet::new();
    let person: Person = serde_ignored::deserialize(json_deserializer, |path| {
        unused.insert(path.to_string());
    })
    .expect("Unable to parse JSON");

    println!("Unused fields: {:?}", unused);

    println!("{:?}", person);
}
