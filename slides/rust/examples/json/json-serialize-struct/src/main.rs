use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct Thing {
    name: String,
    number: i8,
    numbers: Vec<i32>,
}

fn main() {
    let thing = Thing {
        name: String::from("Foo Bar"),
        number: 42,
        numbers: vec![23, 19],
    };
    println!("{:#?}", &thing);
    let serialized = serde_json::to_string(&thing).unwrap();
    println!("{serialized}");
}
