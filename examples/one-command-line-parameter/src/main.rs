
fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 2 {
        eprintln!("Usage: {} value", argv[0]);
        std::process::exit(1);
    }

    let value = &argv[1];

    println!("Value: {}", value);
}
