use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    text: String,
}

fn main() {
    let filename = "data.json";
    let content = std::fs::read_to_string(filename).unwrap();

    let data: Point = serde_json::from_str(&content).unwrap();
    println!("data = {:?}", data);
    println!("{}", data.x + data.y);
    println!("{}", data.text);
    println!();

    // Using the Turbofixh syntax
    let other = serde_json::from_str::<Point>(&content).unwrap();
    println!("other = {:?}", other);

}

