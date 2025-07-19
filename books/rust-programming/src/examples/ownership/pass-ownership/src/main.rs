fn main() {
    let name = String::from("Foo Bar");
    println!("{name}");

    greet(name);

    // We cannot use name any more as it was moved
    //println!("{name}"); // borrow of moved value: `name`
    //greet(name);        // use of moved value: `name`
}

fn greet(text: String) {
    println!("Greet: {text}");
}
