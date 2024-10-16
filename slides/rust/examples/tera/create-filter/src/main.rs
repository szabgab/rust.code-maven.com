use std::collections::HashMap;

use tera::{to_value, Context, Result, Tera, Value};

fn fixed_filter(_val: &Value, _map: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value("some fixed value").unwrap())
}

fn same_value_filter(val: &Value, _map: &HashMap<String, Value>) -> Result<Value> {
    Ok(to_value(val).unwrap())
}

fn my_len(val: &Value, _map: &HashMap<String, Value>) -> Result<Value> {
    let s = val.as_str().unwrap();
    Ok(to_value(s.len()).unwrap())
}

fn filter_with_params(_val: &Value, map: &HashMap<String, Value>) -> Result<Value> {
    //println!("map: {:?}", map);
    let param = map.get("param").unwrap().as_str().unwrap();
    let attr = map.get("attr").unwrap().as_u64().unwrap();

    Ok(to_value(format!("param={param} attr={attr}")).unwrap())
}

fn main() {
    let mut tera = Tera::new("templates/*.html").unwrap();
    tera.register_filter("fixed_filter", fixed_filter);
    tera.register_filter("same_value_filter", same_value_filter);
    tera.register_filter("my_len", my_len);
    tera.register_filter("filter_with_params", filter_with_params);

    let mut context = Context::new();
    context.insert("text", "Hello, World!");

    let result = tera.render("hello.html", &context).unwrap();
    println!("{result}");
}
