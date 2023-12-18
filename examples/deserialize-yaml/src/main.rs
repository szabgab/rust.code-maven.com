use serde::Deserialize;
use std::fs;


#[derive(Debug, Deserialize)]
struct Version {
    ladino: String,
    rashi: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Word {
    grammar: String, // TODO fixed list 
    orijen: String, // TODO
    versions: Vec<Version>,
}

fn main() {
    let filename = get_filename();
    let text = fs::read_to_string(&filename).unwrap();

    let word: Word = serde_yaml::from_str(&text).unwrap_or_else(|err| {
        eprintln!("Could not parse YAML file: {err}");
        std::process::exit(1);
    });

    println!("grammar: {}", word.grammar);
    println!("orijen: {}", word.orijen);
    for version in &word.versions {
        println!("---");
        println!("ladino: {}", version.ladino);
        println!("rashi: {}", version.rashi);
    }


    if filename == "kaza.yaml" {
        assert_eq!(word.grammar, "NA");
        assert_eq!(word.orijen, "Jeneral");
        assert_eq!(word.versions[0].ladino, "kaza");
        assert_eq!(word.versions[1].ladino, "kazas");
    }
}

fn get_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    args[1].to_string()
}
