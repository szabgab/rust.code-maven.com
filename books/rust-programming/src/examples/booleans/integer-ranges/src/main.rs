fn main() {
    let n = get_arg().parse::<u8>().unwrap();
    match n {
        0 => println!("zero"),
        1..=20 => println!("small"),
        21..=255 => println!("big"),
        //        21..= => println!("big"),
        //        _ => println!("other"),

        // 21..=254 => println!("big"),
        // non-exhaustive patterns: `u8::MAX` not covered
        // pattern `u8::MAX` not covered
    }
}

fn get_arg() -> String {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("{} NUMBER", args[0]);
        std::process::exit(1);
    }

    args[1].to_owned()
}
