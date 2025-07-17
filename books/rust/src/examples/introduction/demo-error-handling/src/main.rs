fn main() {
    match read_files() {
        Ok(()) => (),
        Err(err) => {
            println!("Error: {err}");
        }
    }
}

fn read_files() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("other.md")?;
    println!("content length: {}", content.len());
    println!();

    let content = std::fs::read_to_string("src/main.rs")?;
    println!("content length: {}", content.len());

    Ok(())
}
