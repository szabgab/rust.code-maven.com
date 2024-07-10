use std::io;
use std::io::Write;

fn main() {
    let mut name = String::new();

    print!("Please type in your name: ");
    io::stdout().flush().expect("Oups");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to get input");
    name = name.trim_end().to_owned();

    println!("Hello {name}, how are you?");
}
