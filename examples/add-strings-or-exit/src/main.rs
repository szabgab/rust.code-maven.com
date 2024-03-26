fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} NUMBER NUMBER", args[0]);
        std::process::exit(1);
    }
    println!("{}", add(&args[1], &args[2]));
}

fn add(a: &str, b: &str) -> i32 {
    let a = match a.parse::<i32>() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Failed converting the value '{a}': {err}");
            std::process::exit(1);
        }
    };
    let b = match b.parse::<i32>() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Failed converting the value '{b}': {err}");
            std::process::exit(1);
        }
    };

    a + b
}

#[test]
fn check_add_or_exit() {
    assert_eq!(add("2", "3"), 5);
}
