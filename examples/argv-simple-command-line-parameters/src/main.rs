fn main() {
    let argv: Vec<String> = std::env::args().collect();

    println!("{:?}", argv);
    println!("Number of elements on the command line {}", argv.len());

    if argv.len() < 2 {
        eprintln!("Usage: {} value", argv[0]);
        std::process::exit(1);
    }

    println!();
    for param in &argv[1..argv.len()] {
        println!("{}", param);
    }
    println!();

    if argv.len() < 3 {
        std::process::exit(1);
    }

    let first = &argv[1];
    let second = &argv[2];
    println!("First: {}", first);
    println!("Second: {}", second);
}
