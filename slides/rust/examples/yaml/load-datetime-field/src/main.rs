use chrono::{DateTime, Timelike, Utc};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Event1 {
    title: String,
    start: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Event2 {
    title: String,
    start: DateTime<Utc>,
}

fn main() {
    let filename = "data.yaml";
    let content = std::fs::read_to_string(filename).expect("File not found");
    let data: Event1 = serde_yml::from_str(&content).expect("YAML parsing error");
    println!("{:?}", data);

    let data: Event2 = serde_yml::from_str(&content).expect("YAML parsing error");
    println!("{:?}", data);
    println!("hour: {}", data.start.hour());
    println!("timezone: {}", data.start.timezone());
}
