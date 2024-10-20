use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "key_value_pairs.txt";
    let mut data = HashMap::new();
    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let line = line.unwrap();
                // println!("'{}'", line);
                let parts = line.split('=');
                //println!("{:?}", parts);
                let parts: Vec<&str> = parts.collect();
                // println!("{:?}", parts);
                //println!("{:?}", parts[0]);
                data.insert(parts[0].to_owned(), parts[1].to_owned());
            }
        }
        Err(error) => {
            println!("Error opening file {}: {}", filename, error);
        }
    }
    println!("{:?}", data);
}
