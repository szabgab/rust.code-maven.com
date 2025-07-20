fn main() {
    let animals = vec![
        String::from("cat"),
        String::from("dog"),
        String::from("crab"),
    ];

    for animal in &animals {
        println!("{animal}");
    }
    println!();

    for animal in animals {
        println!("{animal}");
    }
}
