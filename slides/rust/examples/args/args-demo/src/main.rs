fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{args:?}");

    if args.len() < 2 {
        eprintln!("Usage: {} param", args[0]);
        std::process::exit(1);
    }
    let param = &args[1];
    println!("{param:?}");

    let number = param.parse::<u32>().expect("We expected a number");
    println!("{number}");
    println!("{}", number + 1);
}
