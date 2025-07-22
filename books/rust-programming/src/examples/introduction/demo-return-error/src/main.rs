fn main() {
    for file in ["a.txt", "b.txt", "c.txt"].iter() {
        match divide(100, file) {
            Ok(result) => println!("100 / divisor = {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn divide(dividend: i32, filename: &str) -> Result<i32, String> {
    let content = std::fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file {}: {}", filename, e))?;

    let divisor = content
        .trim()
        .parse::<i32>()
        .map_err(|_| format!("Failed to parse number in file {}", filename))?;

    if divisor == 0 {
        return Err(format!("Cannot divide by zero in file {}", filename));
    }

    Ok(dividend / divisor)
}
