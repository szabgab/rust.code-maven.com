fn main() {
    let colors = vec!["blue", "red", "green", "yellow"];
    println!("{:?}", colors);

    let colors = vec![
        "blue".to_string(),
        "red".to_string(),
        "green".to_string(),
        "yellow".to_string(),
    ];
    println!("{:?}", colors);

    let colors = vec!["blue", "red", "green", "yellow"]
        .into_iter()
        .map(|str| str.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", colors);
}
