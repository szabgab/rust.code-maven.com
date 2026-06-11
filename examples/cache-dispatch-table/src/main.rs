use cached::macros::cached;

mod locations;
mod other;
mod types;

use crate::types::DispatchTable;

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

#[cached]
fn get_dispatch() -> DispatchTable {
    println!("get_dispatch called");

    let mut config = DispatchTable::new();

    let cfg = crate::locations::get_dispatch();
    for (key, value) in cfg {
        config.insert(key, value);
    }

    let cfg = crate::other::get_dispatch();
    for (key, value) in cfg {
        config.insert(key, value);
    }

    config
}
