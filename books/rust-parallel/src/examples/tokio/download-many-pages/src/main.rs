
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_urls_from_file(path: &str) -> Vec<String> {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|line| {
            let line = line.ok()?.trim().to_string();
            if line.is_empty() || line.starts_with('#') {
                None
            } else {
                Some(line)
            }
        })
        .collect()
}

fn main() {
    let urls = read_urls_from_file("urls.txt");
    println!("URLs: {:?}", urls);
}
