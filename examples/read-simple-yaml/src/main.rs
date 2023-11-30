use std::env;
use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    fname: String,
    lname: String,
    year: u16,
    height: f32,
    married: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let data: Data = match File::open(filename) {
        Ok(file) => match serde_yaml::from_reader(file) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("There was an error parsing the YAML file {}", err);
                std::process::exit(1);
            }
        },
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        }
    };
    println!("{:#?}", &data);
    println!();

    println!("{}", data.fname);
    assert_eq!(data.lname, "Bar");
    assert_eq!(data.year, 2023);
    assert_eq!(data.height, 6.1);
    assert!(data.married);

}
