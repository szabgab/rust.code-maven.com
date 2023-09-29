fn main() {
    let name = "Foo Bar";
    println!("Hello, {}, how are you?", name);
    println!("Hello, '{}', how are you?", name);
    println!("Hello, \"{}\", how are you?", name);
    println!(r#"Hello, "{}", how are you?"#, name);

    println!(r###"Hello, "{}", how are you?"###, name);
}
