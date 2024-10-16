fn main() {
    let n = get_arg().parse::<f32>().unwrap();
    match n {
        0.0 => println!("zero"),
        1.0..=20.0 => println!("small"),
        21.0.. => println!("big"),
        _ => println!("other"),
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
