fn main() {
    let animals = vec![
        (String::from("elephant"), String::from("huge"), 100),
        (String::from("snake"), String::from("long"), 3),
    ];
    println!("{:?}", animals);
    for animal in &animals {
        println!("{} - {} - {}", animal.0, animal.1, animal.2);
    }

    // each field its own variable
    for (animal, size, weight) in &animals {
        println!("{animal} - {size} - {weight}");
    }

    // If we only need one of the values
    for (animal, _, _) in &animals {
        println!("{animal}");
    }

}


