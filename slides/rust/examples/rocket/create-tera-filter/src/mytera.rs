use std::collections::HashMap;

use rocket_dyn_templates::tera::Tera;
use tera::{Result, Value, to_value};

fn my_len(val: &Value, _map: &HashMap<String, Value>) -> Result<Value> {
    let s = val.as_str().unwrap();
    Ok(to_value(s.len()).unwrap())
}

pub fn customize(tera: &mut Tera) {
    tera.register_filter("my_len", my_len);
}
