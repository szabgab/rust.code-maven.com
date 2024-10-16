fn main() {
    let name = String::from("Foo Bar");
    println!("{name}");
    greet(&name);
    println!("{name}");
}

fn greet(text: &str) {
    println!("Greet: {text}");
}
