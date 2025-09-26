fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let num = args[1].parse::<u8>().expect(&generate_error(&args[1]));

    // let num = args[1]
    //     .parse::<u8>()
    //     .unwrap_or_else(|_| panic!("{}", generate_error(&args[1])));

    // let num = args[1]
    //     .parse::<u8>()
    //     .unwrap_or_else(|err| panic!("{err} {}", generate_error(&args[1])));

    println!("Hello, {num}");
}

fn generate_error(num: &str) -> String {
    println!("generate_error");
    format!("{} is not a valid number", num)
}
