fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} PATH", &args[0]);
        std::process::exit(1);
    }

    let path = std::path::PathBuf::from(&args[1]);

    println!("relative: {}", path.as_os_str().to_str().unwrap());
    println!("absolute: {}", path.canonicalize().unwrap().as_os_str().to_str().unwrap());
}
