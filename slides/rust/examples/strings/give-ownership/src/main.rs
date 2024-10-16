fn main() {
    let name = give_ownership();
    println!("{name}");
}

fn give_ownership() -> String {
    String::from("Foo")
}
