use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    number: u32,
}

fn main() {
    let filename = "data.json";
    let content = std::fs::read_to_string(filename).unwrap();
    let data = serde_json::from_str::<Vec<Person>>(&content).expect("JSON parsing error");
    println!("{:#?}", data);
}
