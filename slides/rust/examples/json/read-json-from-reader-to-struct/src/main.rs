use std::fs::File;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Person {
    fname: String,
    lname: String,
    married: bool,
}


fn main() {
    let filename = get_filename();

    let data: Person = match File::open(&filename) {
        Ok(file) => {
             serde_json::from_reader(&file).expect("JSON parsing error")
        },
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        },
    };
    println!("{:#?}", &data);
    assert_eq!(data.fname, "Foo");
    assert_eq!(data.lname, "Bar");
    assert!(data.married);
}


fn get_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    args[1].to_owned()
}

