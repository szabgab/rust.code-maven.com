
#[derive(Debug)]
struct Animal<'a> {
    name: String,
    sound: &'a str,
}


fn main() {
    let cat = get_cat();

    println!("Animal: {:?}", cat);
    println!("name: {}", cat.name);
}

fn get_cat() -> Animal {
    let cat = Animal {
        name: String::from("Ketzele"),
        sound: "miau",
    };
    cat
}
