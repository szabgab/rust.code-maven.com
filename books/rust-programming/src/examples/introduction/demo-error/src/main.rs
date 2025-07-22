fn main() {
    for file in ["a.txt", "b.txt", "c.txt"].iter() {
        divide(100, file);
    }
}

fn divide(dividend: i32, filename: &str) {
    let divisor_result = std::fs::read_to_string(filename);
    // Result<String, Error>

    match divisor_result {
        Ok(content) => {
            let content = content.trim();
            match content.parse::<i32>() {
                Ok(divisor) => {
                    if divisor == 0 {
                        println!("Cannot divide by zero in file {}", filename);
                    } else {
                        println!("{} / {} = {}", dividend, divisor, dividend / divisor);
                    }
                }
                Err(_) => {
                    println!("Failed to parse number in file {}", filename);
                }
            }
        }
        Err(err) => {
            println!("Error: {err}");
        }
    }
}
