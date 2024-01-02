use std::io::Write;

fn main() {
    let name = prompt("What is your name?");
    println!("Your name is '{}'", name);
}


fn prompt(text: &str) -> String {
    print!("{} ", text);
    std::io::stdout().flush().expect("Oups");

    std::io::stdin().lines().next().unwrap().unwrap()
}
