fn main() {
    let animals = vec![
        String::from("cat"),
        String::from("dog"),
        String::from("crab"),
    ];

    let mut iterator = animals.iter();
    loop {
        if let Some(animal) = iterator.next() {
            println!("{animal}")
        } else {
            println!("No more animals");
            break;
        }
    }

    if let Some(animal) = iterator.next() {
        println!("{animal}")
    } else {
        println!("This iterator is finished");
    }
}
