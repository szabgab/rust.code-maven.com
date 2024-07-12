fn main() {
    let result = std::fs::read_to_string("other.md"); // Result<String, Error>

    match result {
        Ok(content) => {
            println!("content length: {}", content.len());
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }

    println!();

    let result = std::fs::read_to_string("src/main.rs"); // Result<String, Error>

    match result {
        Ok(content) => {
            println!("content length: {}", content.len());
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}
