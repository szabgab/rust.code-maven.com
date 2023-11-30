use std::fs::File;

use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct Person {
    fname: String,
    married: bool,
}

fn main() {
    let filename = get_filename();

    let data = match File::open(&filename) {
        Ok(file) => {
             let data: Person = serde_json::from_reader(&file).expect("JSON parsing error");
             data
        },
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        },
    };
    dbg!(&data);
    assert_eq!(data.fname, "Foo");
    assert!(data.married);
}


fn get_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    args[1].to_string()
}


