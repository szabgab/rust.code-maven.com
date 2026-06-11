use crate::types::DispatchTable;

fn process_city() -> &'static str {
    "Processing city of Seoul"
}

fn process_country() -> &'static str {
    "Processing country of Korea"
}

pub(crate) fn get_dispatch() -> DispatchTable {
    println!("get_dispatch for locations called");

    let mut config = DispatchTable::new();
    config.insert("city", process_city);
    config.insert("country", process_country);
    config
}
