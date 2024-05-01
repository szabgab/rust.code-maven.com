use std::collections::HashSet;

fn main() {
    let mut english: HashSet<String> = HashSet::new();
    println!("{:?}", &english);

    english.insert(String::from("chair"));
    println!("{:?}", &english);

    english.insert(String::from("table"));
    println!("{:?}", &english);

    english.insert(String::from("chair"));
    println!("{:?}", &english);

}