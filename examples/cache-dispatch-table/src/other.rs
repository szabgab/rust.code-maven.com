use crate::types::DispatchTable;

fn process_language() -> &'static str {
    "Processing the Korean language"
}

pub(crate) fn get_dispatch() -> DispatchTable {
    println!("get_dispatch for locations called");

    let mut config = DispatchTable::new();
    config.insert("language", process_language);
    config
}
