use std::io;

fn main() {
    let mut name = String::new();

    println!("Please type in your name:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get input");

    name = name.trim_end().to_string();

    println!("Hello {}, how are you?", name);
}

