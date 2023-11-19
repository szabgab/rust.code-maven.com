use std::collections::HashMap;

fn main() {
    let ext_to_languages = get_languages();

    println!("{:?}", ext_to_languages);
    println!("{:?}", ext_to_languages["rs"]);

    assert_eq!(ext_to_languages["rs"], "rust");
}

fn get_languages() -> HashMap<String, String> {
    let text = include_str!("../data/languages.csv");

    text.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (key, value) = line.split_once(',').expect("no comma? no results");
            (key.to_owned(), value.to_owned())
        }).collect::<HashMap<String, String>>()
}

