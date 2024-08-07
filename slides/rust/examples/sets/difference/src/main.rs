use std::collections::HashSet;

fn main() {
    let mut english: HashSet<String> = HashSet::new();
    let mut spanish: HashSet<String> = HashSet::new();

    for word in ["door", "car", "lunar", "era"] {
        english.insert(word.to_owned());
    }
    for word in ["era", "lunar", "hola"] {
        spanish.insert(word.to_owned());
    }

    println!("{:?}", &english);
    println!("{:?}", &spanish);

    println!("{:?}", english.difference(&spanish));
    println!("{:?}", spanish.difference(&english));
}
