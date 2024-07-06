fn main() {
    let name = String::from("Foo");
    println!("{name}");
    borrow(&name);
    println!("{name}");

}

fn borrow(name: &str) {
    println!("in function: {name}");
}

