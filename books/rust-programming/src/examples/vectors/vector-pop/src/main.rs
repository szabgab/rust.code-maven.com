fn main() {
    let mut animals = vec![
        String::from("dog"),
        String::from("cat"),
        String::from("camel"),
        String::from("crab"),
        String::from("snake"),
    ];
    println!("{} {}", animals.len(), animals.capacity());
    match animals.pop() {
        None => println!("No animals"),
        Some(last) => println!("last: {last}"),
    }
    println!("{} {}", animals.len(), animals.capacity());

    animals.shrink_to_fit();
    println!("{} {}", animals.len(), animals.capacity());
}
