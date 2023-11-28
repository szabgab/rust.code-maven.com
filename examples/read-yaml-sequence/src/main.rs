use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    let data: serde_yaml::Value = match File::open(filename) {
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

    println!("{:#?}", &data[0]);
    let name = data[0].get("name").unwrap().as_str().unwrap();
    println!("{}", name);
    assert_eq!(name, "Foo");
    println!();

    println!("{:#?}", &data[1]);
    let name = data[1].get("name").unwrap().as_str().unwrap();
    let year = data[1].get("year").unwrap().as_u64().unwrap();
    let married = data[1].get("married").unwrap().as_bool().unwrap();
    println!("{}", name);
    println!("{}", year);
    println!("{}", married);
    assert_eq!(name, "Bar");
    assert_eq!(year, 2023);
    assert!(married);
    println!();
}
