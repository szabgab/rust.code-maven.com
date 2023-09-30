
fn main() {
    let argv: Vec<String> = std::env::args().collect();
    if argv.len() != 2 {
        eprintln!("Usage: {} value", argv[0]);
        std::process::exit(1);
    }

    let value = &argv[1];

    let num: i16 = value.parse().expect("Could not convert");
    //let num = value.parse::<i16>().expect("Could not convert");

    //let num: i16 = value.parse().unwrap();

    //let num: i16 = value.parse().unwrap_or(42);

    //let num: i16 = value.parse().unwrap_or_default();


    // let num: i16 = match value.parse() {
    //     Ok(val) => val,
    //     Err(err) => {
    //         eprintln!("Could not convert: '{err}'");
    //         return;
    //     }
    // };

    println!("Converted to {}", num);
}
