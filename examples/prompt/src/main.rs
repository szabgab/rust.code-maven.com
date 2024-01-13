use std::io::Write;

fn main() {
    let name = prompt("What is your name?");
    println!("Your name is '{}'", name);
}

fn prompt(text: &str) -> String {
    print!("{} ", text);
    std::io::stdout().flush().expect("Oups");

    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Failed to get input");

    response.trim_end().to_string()
}
