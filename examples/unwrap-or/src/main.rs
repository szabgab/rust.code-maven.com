fn main() {
    using_match();

    //using_unwrap();
}

fn using_unwrap() {
    //let input = "42";
    let input = "42.2";


    //let number = input.parse::<u32>().unwrap();
    //let number = input.parse::<u32>().unwrap_or(23);
    //let number = input.parse::<u32>().unwrap_or_default();

    let number = input.parse::<u32>().unwrap_or_else(|err| {
        eprintln!("{}", err);
        12
    });
    // Will set the function returns.

    println!("number: {}", number);
}

fn using_match() {
    //let input = "42";
    let input = "42.2";

    match input.parse::<u32>() {
        Ok(val) => {
            println!("{}", val);
        },
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    };
}
