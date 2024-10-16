use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Serialize;

// https://docs.rs/chrono/0.4.19/chrono/serde/index.html

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct Thing {
    name: String,
    number: i8,
    numbers: Vec<i32>,

    #[serde(with = "ts_seconds")]
    now: DateTime<Utc>,
}

fn main() {
    let thing = Thing {
        name: String::from("Foo Bar"),
        number: 42,
        numbers: vec![23, 19],
        now: Utc::now(),
    };
    println!("{:#?}", &thing);
    let serialized = serde_json::to_string(&thing).unwrap();
    println!("{}", serialized);
}
