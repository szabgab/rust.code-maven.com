fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} FILENAME", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    println!("filename {filename}");

    let data: Vec<u8> = std::fs::read(filename).unwrap();
    println!("len: {}", data.len());
}
