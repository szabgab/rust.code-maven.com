use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Thing {
    name: String,
    // rust-version: String,

    #[serde(alias = "rust-version")]
    rust_version: String,
}

fn main() {
    let filename = "data.json";
    let content = std::fs::read_to_string(filename).unwrap();
    let data = serde_json::from_str::<Thing>(&content).expect("JSON parsing error");
    println!("{:#?}", data);
}
