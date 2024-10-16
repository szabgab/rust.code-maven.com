fn main() {
    let mut animals = get_animals();

    println!("{animals:?}");
    assert_eq!(animals, ["snake", "crab", "elephant", "lizard"]);

    animals.sort();
    println!("{animals:?}");
    assert_eq!(animals, ["crab", "elephant", "lizard", "snake"]);

    #[allow(clippy::unnecessary_sort_by)]
    animals.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{animals:?}");
    assert_eq!(animals, ["crab", "snake", "lizard", "elephant"]);

    let mut animals = get_animals();
    animals.sort_by_key(|a| a.len());
    println!("{animals:?}");
    assert_eq!(animals, ["crab", "snake", "lizard", "elephant"]);
}

fn get_animals() -> Vec<String> {
    vec![
        String::from("snake"),
        String::from("crab"),
        String::from("elephant"),
        String::from("lizard"),
    ]
}
