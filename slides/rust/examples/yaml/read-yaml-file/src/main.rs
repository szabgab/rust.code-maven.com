use std::fs::File;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    text: String,
}

fn main() {
    read_any_yaml();
    read_struct_yaml();
}

fn read_any_yaml() {
    let filename = "data.yaml";
    match File::open(filename) {
        Ok(file) => {
            let data: serde_yml::Value = serde_yml::from_reader(file).expect("YAML parsing error");
            dbg!(&data);

            let text = match data.get("text") {
                Some(val) => val.as_str().unwrap(),
                None => panic!("Field text does not exist"),
            };
            println!("{}", text);

            let x = match data.get("x") {
                Some(val) => val.as_i64().unwrap(),
                None => panic!("Field x does not exist"),
            };
            let y = match data.get("y") {
                Some(val) => val.as_i64().unwrap(),
                None => panic!("Field y does not exist"),
            };
            println!("{}", x + y);
        }
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        }
    }
}

fn read_struct_yaml() {
    let filename = "data.yaml";
    match File::open(filename) {
        Ok(file) => {
            let data: Point = serde_yml::from_reader(file).unwrap();
            println!("data = {:?}", data);
            println!("{}", data.x + data.y);
            println!("{}", data.text);
            assert_eq!(data.x, 1);
            assert_eq!(data.y, 2);
            assert_eq!(data.text, "Hello World!");
        }
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        }
    }
}
