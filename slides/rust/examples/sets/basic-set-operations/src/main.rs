use std::collections::HashSet;

fn main() {
    let mut english: HashSet<String> = HashSet::new();
    println!("{:?}", &english);

    english.insert(String::from("chair"));
    println!("{:?}", &english);

    english.insert(String::from("table"));
    println!("{:?}", &english);

    english.insert(String::from("chair".));
    println!("{:?}", &english);

    println!("{}", english.contains("chair"));
    println!("{}", english.contains("door"));
    println!("{}", english.len());

    for word in &english {
        println!("{}", word);
    }

    english.remove("chair");
    println!("{:?}", &english);
}
