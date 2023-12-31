use std::io::Write;

fn main() {
    let name = prompt("What is your name?");
    println!("Your name is '{}'", name);

    let name = prompt2("What is your name?");
    println!("Your name is '{}'", name);
}

fn prompt(text: &str) -> String {
    print!("{} ", text);
    std::io::stdout().flush().expect("Oups");

    let mut response = String::new();
    std::io::stdin()
        .read_line(&mut response)
        .expect("Faild to get input");

    response.trim_end().to_string()
}

fn prompt2(text: &str) -> String {
    print!("{} ", text);
    std::io::stdout().flush().expect("Oups");

    std::io::stdin().lines().next().unwrap().unwrap()
}
