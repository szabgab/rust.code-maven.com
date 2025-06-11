use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
#[serde(untagged)]
enum Readme {
    Bool(bool),
    String(String),
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    readme: Readme,
}
fn main() {
    let foo_file = "foo.json";
    let content = std::fs::read_to_string(foo_file).unwrap();
    let data = serde_json::from_str::<Person>(&content).unwrap();
    println!("{:?}", data);

    let bar_file = "bar.json";
    let content = std::fs::read_to_string(bar_file).unwrap();
    let data = serde_json::from_str::<Person>(&content).unwrap();
    println!("{:?}", data);
}
