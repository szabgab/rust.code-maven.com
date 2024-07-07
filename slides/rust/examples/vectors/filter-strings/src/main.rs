fn main() {
    let animals: Vec<String> = vec![
        String::from("elephant"),
        String::from("cat"),
        String::from("snake"),
        String::from("dog"),
    ];
    dbg!(&animals);

    let same_animals: Vec<String> = animals.iter().filter(|_animal| true).cloned().collect();
    dbg!(&same_animals);

    let short_animals: Vec<String> = animals.iter().filter(|animal| animal.len() < 4).cloned().collect();
    dbg!(&short_animals);
}
