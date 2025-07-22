fn main() {
    for file in ["a.txt", "b.txt", "c.txt"].iter() {
        match divide(100, file) {
            Ok(result) => println!("100 / divisor = {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
}

fn divide(dividend: i32, filename: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(filename)?;

    let divisor = content.trim().parse::<i32>()?;

    if divisor == 0 {
        return Err("Division by zero is not allowed".into());
    }
    Ok(dividend / divisor)
}
