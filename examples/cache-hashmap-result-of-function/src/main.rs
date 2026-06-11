use std::collections::HashMap;
use cached::macros::cached;

fn main() {
    let config = get_config();
    println!("Config: {:?}", config);

    let config = get_config();
    println!("Config: {:?}", config);
}


#[cached]
fn get_config() -> HashMap<&'static str, String> {
    println!("get_config called");

    let mut config = HashMap::new();
    config.insert("city", String::from("Seoul"));
    config.insert("country", String::from("Korea"));
    config
}



