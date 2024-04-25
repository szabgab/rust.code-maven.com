use std::collections::HashMap;

fn main() {
    let mut length_of: HashMap<String, u32> = HashMap::new();
    println!("{}", length_of.len());
    println!("{:#?}", length_of);
    println!();

    length_of.insert(String::from("snake"), 320);
    println!("{}", length_of.len());
    println!("{:#?}", length_of);
    println!();

    length_of.insert(String::from("snail"), 4);
    println!("{}", length_of.len());
    println!("{:#?}", length_of);

}
