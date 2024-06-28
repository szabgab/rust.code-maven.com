use std::error::Error;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Animal {
    #[serde(default="cat")]
    Cat,
    Dog,
}

#[derive(Serialize, Deserialize)]
struct Pet {
    name: String,
    class: Animal,
}

fn main() -> Result<(), Box<dyn Error>> {
    let my_dog = Pet {
        name: String::from("Chewy"),
        class: Animal::Dog,
    };

    let my_cat = Pet {
        name: String::from("Kitty"),
        class: Animal::Cat,
    };

    println!("{}", serde_json::to_string_pretty(&my_dog)?);
    println!("{}", serde_json::to_string_pretty(&my_cat)?);

    Ok(())
}
