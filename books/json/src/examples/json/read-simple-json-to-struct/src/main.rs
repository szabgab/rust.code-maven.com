use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
    f: f64,
    text: String,
}

fn main() {
    let filename = "data.json";
    let content = std::fs::read_to_string(filename).unwrap();

    let data: Point = serde_json::from_str(&content).unwrap();
    println!("data = {:?}", data);
    println!("x+y:  {}", data.x + data.y);
    println!("f:    {}", data.f);
    println!("text: {}", data.text);
    println!();

    // Using the Turbofish syntax
    let other = serde_json::from_str::<Point>(&content).unwrap();
    println!("other = {:?}", other);
}
