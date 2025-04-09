fn main() {
    let filename = "data.json";
    let content = std::fs::read_to_string(filename).unwrap();

    let animals: Vec<String> = serde_json::from_str(&content).unwrap();
    println!("animals:   {animals:?}");

    // doing the same using Turbofish
    let turbofish = serde_json::from_str::<Vec<String>>(&content).unwrap();
    println!("turbofish: {turbofish:?}");

    assert_eq!(animals, turbofish);
}
