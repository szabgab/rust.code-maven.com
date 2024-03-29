use std::io;
use std::io::Write;

fn main() {
    println!("Before");
    wait_for_enter().unwrap();
    println!("After");
}

fn wait_for_enter() -> Result<(), std::io::Error> {
    print!("Press ENTER to continue!");
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;

    Ok(())
}
