fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} NUMBER NUMBER", args[0]);
        std::process::exit(1);
    }
    println!("{}", add(&args[1], &args[2]));
}

fn add(a: &str, b: &str) -> i32 {
    let a = a.parse::<i32>().unwrap();
    let b = b.parse::<i32>().unwrap();
    a + b
}

#[test]
fn check_add() {
    assert_eq!(add("2", "3"), 5);
}
