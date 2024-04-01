use std::collections::HashMap;

fn main() {
    let mut count: HashMap<String, u32> = HashMap::new();
    add(&mut count, &["camel", "snake", "crab", "crab"]);
    add(&mut count, &["camel", "camel", "crab", "crab"]);

    println!("{:#?}", count);
}

fn add(counter: &mut HashMap<String, u32>, words: &[&str]) {
    for word in words {
        *counter.entry(word.to_owned().to_owned()).or_insert(0) += 1;
    }
}

