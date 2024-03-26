fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} NUMBER NUMBER", args[0]);
        std::process::exit(1);
    }

    match add(&args[1], &args[2]) {
        Ok(val) => println!("{val}"),
        Err(err) => {
            eprintln!("{err}");
            std::process::exit(1);
        }
    };
}

fn add(a: &str, b: &str) -> Result<i32, std::num::ParseIntError> {
    let a = a.parse::<i32>()?;
    let b = b.parse::<i32>()?;
    Ok(a + b)
}

#[test]
fn check_add_or_exit() {
    assert_eq!(add("2", "3"), Ok(5));
    assert_eq!(
        add("2.1", "3").unwrap_err().to_string(),
        "invalid digit found in string"
    );
    assert_eq!(
        add("2", "3.2").unwrap_err().to_string(),
        "invalid digit found in string"
    );
}
