fn main() {
    let mut text = String::new();
    println!("{text:?}");

    text.push_str("Hello");
    println!("{text:?}");

    text.push(' ');
    println!("{text:?}");

    text.push_str("World");
    println!("{text:?}");

    text.push('!');
    println!("{text:?}");
}
