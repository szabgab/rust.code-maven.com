use std::collections::HashMap;


fn main() {
    let animals = HashMap::from([
        ("cat", 2),
        ("dog", 3),
        ("snake", 4),
        ("crab", 5),
    ]);
    println!("animals: {animals:?}");

    for pair in &animals {
        println!("{pair:?}");
    }

    let mut animals = animals.iter().collect::<Vec<_>>();
    animals.sort();
    println!("animals: {animals:?}");

    animals.sort_by_key(|entry| entry.1);
    println!("animals: {animals:?}");

}
