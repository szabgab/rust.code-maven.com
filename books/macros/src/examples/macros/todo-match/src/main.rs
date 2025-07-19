fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} NAME", args[0]);
        std::process::exit(0);
    }

    match args[1].as_str() {
        "foo" => {
            println!("We are handling foo")
        }
        "bar" => {
            println!("We are handling bar")
        }
        name => todo!("We still need to implement for {name}"),
    };
}
