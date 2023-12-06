fn main() {
    let value = get_command_line_string();

    println!("Value: '{}'", value);
}

fn get_command_line_string() -> String {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() != 2 {
        eprintln!("Usage: {} value", argv[0]);
        std::process::exit(1);
    }

    argv[1].to_owned()
}
