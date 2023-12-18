use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Person {
    name: String,

    #[serde(default = "get_default_email")]
    email: String,

    #[serde(default = "get_default_year")]
    year: u32,

    #[serde(default = "get_default_married")]
    married: bool,
}

fn get_default_email() -> String {
    String::from("default@address")
}

fn get_default_year() -> u32 {
    2000
}

fn get_default_married() -> bool {
    false
}

fn main() {
    let filename = get_filename();
    let text = fs::read_to_string(filename).unwrap();

    let data: Person = serde_yaml::from_str(&text).unwrap_or_else(|err| {
        eprintln!("Could not parse YAML file: {err}");
        std::process::exit(1);
    });

    println!("name: {}", data.name);
    println!("email: {}", data.email);
    println!("year: {}", data.year);
    println!("married: {}", data.married);
}

fn get_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    args[1].to_string()
}
