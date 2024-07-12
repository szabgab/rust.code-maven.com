use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Thing {
    name: String,
    number: u32,
}

fn main() {
    let filename = "data.json";
    let content = std::fs::read_to_string(filename).unwrap();
    for row in content.split('\n') {
        if row.is_empty() {
            continue;
        }
        //println!("row: {row}");
        let data = serde_json::from_str::<Thing>(&row).unwrap();
        println!("data: {data:#?}");
    }
}
