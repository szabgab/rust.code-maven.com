fn main() {
    let number = get_command_line_number();
    println!("number: {}", number);
    println!("commafied: {}", commafy(number));
}

fn commafy<Integer: Into<i128> + Copy + std::fmt::Debug + std::fmt::Display>(
    number: Integer,
) -> String {
    let num = number.into().abs();

    let num = format!("{num}");

    let mut ix = 0;

    let num = num
        .chars()
        .rev()
        .map(|chr| {
            ix += 1;
            if ix % 3 == 1 && ix > 1 {
                format!(",{chr}")
            } else {
                format!("{chr}")
            }
        })
        .collect::<String>();

    let prefix = if number.into() < 0 { "-" } else { "" };

    format!("{}{}", prefix, num.chars().rev().collect::<String>())
}

fn get_command_line_number() -> i32 {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() != 2 {
        eprintln!("Usage: {} value", argv[0]);
        std::process::exit(1);
    }

    let value: i32 = match argv[1].parse() {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Could not convert '{}' to i32. {}", argv[1], err);
            std::process::exit(1);
        }
    };

    value
}


#[test]
fn test_commafy() {

    assert_eq!(commafy(0), "0");
    assert_eq!(commafy(23), "23");
    assert_eq!(commafy(123), "123");
    assert_eq!(commafy(1234), "1,234");
    assert_eq!(commafy(-23), "-23");
    assert_eq!(commafy(-1234), "-1,234");

    assert_eq!(commafy(23i128), "23");
    assert_eq!(commafy(23u64), "23");
    //assert_eq!(commafy(23u128), "23");
}
