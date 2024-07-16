fn main() {
    greet("Foo");
    greet("Bar");
    println!();

    let name = "Foo";
    greet(name);
    greet(name);
    println!();

    let name = String::from("Bar");
    greet(&name);
    greet(&name);
}

fn greet(name: &str) {
    println!("Hello {}!", name);
}
