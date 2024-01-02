use std::io::Write;

fn main() {
    let result = prompt("What is your name?");
    match result {
        Ok(name) => println!("Your name is '{}'", name),
        Err(err) => {
            eprintln!("There was an error '{}'", err);
            std::process::exit(1);
        }
    };
}

fn prompt(text: &str) -> Result<String, std::io::Error> {
    print!("{} ", text);
    std::io::stdout().flush()?;

    let mut response = String::new();
    std::io::stdin().read_line(&mut response)?;

    Ok(response.trim_end().to_string())
}
