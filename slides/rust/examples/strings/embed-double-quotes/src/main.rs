fn main() {
    let name = String::from("Foo");

    println!("Hello {name}, how are you?");
    println!("Hello '{name}', how are you?");
    println!("Hello \"{name}\", how are you?");
    println!(r#"Hello "{name}", how are you?"#);
}
