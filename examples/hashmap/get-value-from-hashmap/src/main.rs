use std::collections::HashMap;

fn main() {
    let mut animal = HashMap::new();


    animal.insert(String::from("snake"), String::from("long"));
    animal.insert(String::from("giraffe"), String::from("tall"));
    animal.insert(String::from("elephant"), String::from("heavy"));

    println!("{:#?}", animal);

    let snake = &animal["snake"];
    println!("snake: {snake}");

    // panic: no entry found for key
    // let turtle = &animal["turtle"];
    // println!("turtle: {turtle}");

    for name in ["turle", "snake"] {
        let value = animal.get(name);
        match value {
            Some(val) => println!("The value of {name} is {val}"),
            None => println!("The key {name} does not exist in the HashMap"),
        }
    }

    value_is_option();
}

fn value_is_option() {
    let phone = HashMap::from([
        ("Joe", Some("123")),
        ("Jane", None),
    ]);

    let val = phone.get("Jane");
    println!("{val:?}"); // Some(None)

    let val = phone.get("Mary");
    println!("{val:?}"); // None
}
