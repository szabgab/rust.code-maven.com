fn main() {
    let mut animals = vec![
        String::from("snake"),
        String::from("crab"),
        String::from("camel"),
        String::from("elephant"),
        String::from("lizard"),
    ];
    println!("{animals:?}");
    animals.sort();
    println!("{animals:?}");

    animals.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{animals:?}");
}
