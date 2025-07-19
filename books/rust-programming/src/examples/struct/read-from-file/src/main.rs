use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
#[allow(dead_code)]
struct Animal {
    name: String,
    legs: u8,
}

fn main() {
    let animals = read_file();
    println!("{:?}", animals);
}

fn read_file() -> Result<Vec<Animal>, Box<dyn Error>> {
    let filename = "animals.txt";
    let fh = File::open(filename)?;
    let reader = BufReader::new(fh);
    let mut animals = vec![];
    for line in reader.lines() {
        let line = line?;
        let pieces = line.split(',').collect::<Vec<_>>();
        animals.push(Animal {
            name: pieces[0].to_string(),
            legs: pieces[1].parse()?,
        })
    }

    Ok(animals)
}
