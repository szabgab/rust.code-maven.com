fn main() {
    let value = get_command_line_number();

    println!("Value: '{}'", value);
}

fn get_command_line_number() -> i32 {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() != 2 {
        eprintln!("Usage: {} value", argv[0]);
        std::process::exit(1);
    }

    let value: i32 = match argv[1].parse() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Could not convert '{}' to i32. {}", argv[1], err);
            std::process::exit(1);
        }
    };

    value
}
