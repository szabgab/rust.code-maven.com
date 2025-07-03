fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} WIDTH HEIGHT", args[0]);
        std::process::exit(2);
    }
    let x = args[1].parse::<u32>().unwrap();
    let y = args[2].parse::<u32>().unwrap();
    println!("{}", x * y);
}
