fn main() {
    let number = get_args();
    dbg!(number);
}

fn get_args() -> u32 {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() != 2 {
        usage(&args[0]);
    }

    let number: u32 = match args[1].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Invalid parameter: '{}'. It must be an integer", err);
            usage(&args[0]);
        }
    };

    number
}

fn usage(name: &str) -> ! {
    eprintln!("Usage: {} INTEGER", name);
    std::process::exit(1);
}
