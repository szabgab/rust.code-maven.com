use std::collections::HashMap;
use cached::macros::cached;

fn main() {
    for name in &["city", "state", "country", "language"] {
        process(name);
    }     
}

fn process(name: &str) {
    let dispatch = get_dispatch();

    println!("Processing: {name}");
    if let Some(process_fn) = dispatch.get(name) {
        let result = process_fn();
        println!("{}", result);
    }   
    println!(); 
}

type Handler = fn() -> &'static str;
type DispatchTable = HashMap<&'static str, Handler>;


#[cached]
fn get_dispatch() -> DispatchTable {
    println!("get_dispatch called");

    let mut config = DispatchTable::new();
    config.insert("city", process_city);
    config.insert("country", process_country);
    config.insert("language", process_language);
    config
}


fn process_city() -> &'static str {
    "Processing city of Seoul"
}

fn process_country() -> &'static str {
    "Processing country of Korea"
}

fn process_language() -> &'static str {
    "Processing the Korean language"
}
