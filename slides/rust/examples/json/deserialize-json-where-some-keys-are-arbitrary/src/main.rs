use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Platform {
    linux,
    ubuntu,
    windows,
    macos,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Job {
    #[serde(rename = "runs-on")]
    runs_on: Platform,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Config {
    title: String,
    jobs: HashMap<String, Job>,
}

fn main() {
    let filename = "data.json";

    read_any_json(filename);
    read_struct_json(filename);
}

fn read_any_json(filename: &str) {
    let content = std::fs::read_to_string(filename).expect("File not found");
    let data: serde_json::Value = serde_json::from_str(&content).expect("JSON parsing error");
    println!("{:#?}", &data);
    println!("--------");

    let title = match data.get("title") {
        Some(val) => val.as_str().unwrap(),
        None => panic!("Field text does not exist"),
    };
    println!("title: {title}");

    let jobs = match data.get("jobs") {
        Some(val) => val.as_object().unwrap(),
        None => panic!("Field jobs does not exist"),
    };
    println!("{:#?}", &jobs);
    for (key, value) in jobs.iter() {
        println!("key: {:?}  value: {:?}", key, value);
    }
    println!("--------");
}

fn read_struct_json(filename: &str) {
    let content = std::fs::read_to_string(filename).expect("File not found");
    let data: Config = serde_json::from_str(&content).expect("JSON parsing error");
    println!("data = {:?}", data);
    println!("title: {}", data.title);
    println!("{:?}", data.jobs.keys());
    for (key, value) in data.jobs.iter() {
        println!("key: {:?} {:?}", key, value);
    }
    assert_eq!(data.title, "Sample file");
    assert_eq!(data.jobs["test"].runs_on, Platform::ubuntu);
    assert_eq!(data.jobs["build"].runs_on, Platform::windows);
}
