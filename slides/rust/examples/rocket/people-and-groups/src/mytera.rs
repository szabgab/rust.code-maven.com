use std::collections::HashMap;

use rocket_dyn_templates::tera::Tera;
use tera::{to_value, Result, Value};

fn object2id(val: &Value, _map: &HashMap<String, Value>) -> Result<Value> {
    let s = val["id"]["String"].as_str().unwrap();
    Ok(to_value(s).unwrap())
}

pub fn customize(tera: &mut Tera) {
    tera.register_filter("object2id", object2id);
}
