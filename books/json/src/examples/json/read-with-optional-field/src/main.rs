use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Person {
    name: String,

    #[serde(default = "get_default_language")]
    language: String,

    married: Option<bool>,
}

fn get_default_language() -> String {
    String::from("Rust")
}

fn main() {
    let filename = get_filename();

    let content = std::fs::read_to_string(filename).unwrap();

    let data = serde_json::from_str::<Person>(&content).expect("JSON parsing error");
    println!("{:#?}", data);

    match data.married {
        None => println!("We don't know if {} is married or not", data.name),
        Some(val) => println!("Marrige status: {val}"),
    }
}

fn get_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    args[1].to_owned()
}
