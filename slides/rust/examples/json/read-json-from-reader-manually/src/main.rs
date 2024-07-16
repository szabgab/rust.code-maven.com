use std::fs::File;

fn main() {
    let filename = get_filename();

    let data = match File::open(&filename) {
        Ok(file) => {
            let data: serde_json::Value =
                serde_json::from_reader(&file).expect("JSON parsing error");
            data
        }
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            std::process::exit(1);
        }
    };
    println!("{:#?}", &data);
    assert_eq!(data.get("fname").unwrap().as_str().unwrap(), "Foo");

    assert_eq!(data["lname"].as_str().unwrap(), "Bar");
    assert_eq!(data["height"].as_f64().unwrap(), 178.2);
    assert_eq!(data["year"].as_u64().unwrap(), 1992);
    assert_eq!(data["numbers"].as_array().unwrap().len(), 3);
    assert_eq!(data["numbers"][0].as_u64().unwrap(), 23);
    assert_eq!(data["married"].as_bool().unwrap(), true); // maybe better to use assert!
    assert_eq!(data["children"].as_array().unwrap().len(), 2);
    assert_eq!(data["children"][0]["name"].as_str().unwrap(), "Alpha");
}

fn get_filename() -> String {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    args[1].to_owned()
}
