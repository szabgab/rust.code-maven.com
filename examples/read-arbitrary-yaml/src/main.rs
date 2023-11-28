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

    let fname = data["fname"].as_str().unwrap();
    assert_eq!(fname, "Foo");
    assert_eq!(&data["fname"], "Foo");

    for field in ["fname", "lname", "address", "year", "height"] {
        println!("field: {}", field);
        let value = match data.get(field) {
            Some(val) => val,
            None => continue,
        };

        if field == "fname" || field == "lname" {
            println!("{}={}", field, &data[field].as_str().unwrap());
            println!("{}={}", field, value.as_str().unwrap());
        }
        if field == "year" {
            println!("{}={}", field, &data[field].as_u64().unwrap());
            println!("{}={}", field, value.as_u64().unwrap());
        }
        if field == "height" {
            println!("{}={}", field, &data[field].as_f64().unwrap());
            println!("{}={}", field, value.as_f64().unwrap());
        }
    }
    println!();

    let field = "year";
    let value = data.get(field).unwrap().as_u64().unwrap();
    println!("{}={}", field, value);
    assert_eq!(value, 2023);
    println!();

    let field = "height";
    let value = match data.get(field) {
        Some(val) => val.as_f64().unwrap(),
        None => return,
    };
    assert_eq!(value, 6.1);
    println!("height={}", value);

    println!();
    // Iterate over list of values from the YAML file
    let numbers = data.get("numbers").unwrap();
    println!("{:?}", numbers);
    println!("{:?}", numbers.as_sequence().unwrap());

    for num in numbers.as_sequence().unwrap() {
        println!("{}", num.as_u64().unwrap());
    }

    println!();
    // Iterate over list of values hashes the YAML file
    let children = data.get("children").unwrap();
    for child in children.as_sequence().unwrap() {
        println!("child: {:?}", child);
        println!("name: {}", child.get("name").unwrap().as_str().unwrap());
        println!(
            "birthdate: {}",
            child.get("birthdate").unwrap().as_u64().unwrap()
        );
    }
}

