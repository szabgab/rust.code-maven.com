fn main() {
    unwrap_on_result("42");
    // unwrap_on_result("4.2");  
    // panic: called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }

    match_on_result("42");
    match_on_result("4.2");

    unwrap_option("cat");
    // unwrap_option("dog");
    // panic: called `Option::unwrap()` on a `None` value

    match_on_option("cat");
    match_on_option("dog");
}

fn unwrap_on_result(input: &str) {
    let number = input.parse::<u32>().unwrap(); // Result<u32, ParseIntError>
    println!("Input number: {number}");
}

fn match_on_result(input: &str) {
    match input.parse::<u32>() {
        Ok(number) => println!("Input number: {number}"),
        Err(err) => println!("Error: {} when trying to parse '{}'", err, input),
    };
}

fn unwrap_option(animal: &str) {
    let text = "The black cat climbed the green tree";
    let location = text.find(animal).unwrap(); // Option<usize>
    println!("Location of {animal}: {location}");
}

fn match_on_option(animal: &str) {
    let text = "The black cat climbed the green tree";
    match text.find(animal) {
        Some(location) => println!("Location of {animal}: {location}"),
        None => println!("None received - no {animal} found"),
    };
}
