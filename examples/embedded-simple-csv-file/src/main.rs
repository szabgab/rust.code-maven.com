use std::collections::HashMap;

fn main() {
    let ext_to_languages = get_languages();

    println!("{:?}", ext_to_languages);
    println!("{:?}", ext_to_languages["rs"]);

    assert_eq!(ext_to_languages["rs"], "rust");
}

fn get_languages() -> HashMap<String, String> {
    let text = include_str!("../data/languages.csv");

    let mut data = HashMap::new();
    for line in text.split('\n') {
        if line.is_empty() {
            continue;
        }
        let parts = line.split(',');
        let parts: Vec<&str> = parts.collect();
        // let parts = parts.collect::<Vec<&str>>();
        data.insert(parts[0].to_string(), parts[1].to_string());
    }

    data
}

