fn main() {
    let filename = "data.json";
    let content = std::fs::read_to_string(filename).unwrap();
    let data: serde_json::Value = serde_json::from_str(&content).expect("JSON parsing error");
    println!("data: {}", data);

    let text = match data.get("text") {
        Some(val) => val.as_str().unwrap(),
        None => panic!("Field text does not exist"),
    };
    println!("text: {text}");

    let x = match data.get("x") {
        Some(val) => val.as_i64().unwrap(),
        None => panic!("Field x does not exist"),
    };
    println!("x: {x}");
    let y = match data.get("y") {
        Some(val) => val.as_i64().unwrap(),
        None => panic!("Field y does not exist"),
    };
    println!("y: {y}");

    println!("x+y = {}", x+y);
}

