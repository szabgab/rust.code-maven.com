use std::collections::HashMap;

fn main() {
    let mut animal = HashMap::new();


    animal.insert(String::from("snake"), String::from("long"));
    animal.insert(String::from("giraffe"), String::from("tall"));
    animal.insert(String::from("elephant"), String::from("heavy"));

    println!("{:#?}", animal);
    for name in ["turle", "snake"] {
        if animal.contains_key(name) {
            println!("{name} is in the HashMap and it is {}", animal[name]);
        } else {
            println!("{name} is NOT in the HashMap");
        }
    }
}
