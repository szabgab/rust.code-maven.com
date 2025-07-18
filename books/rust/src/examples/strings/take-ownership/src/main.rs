fn main() {
    let name = String::from("Foo");
    println!("{name}");
    take_ownership(name);
    //println!("{name}"); // take_ownership moved the owner
}

fn take_ownership(name: String) {
    println!("in function: {name}");
}
