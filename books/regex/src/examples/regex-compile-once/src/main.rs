use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    let rows = vec![
        String::from("Hello, world!"),
        String::from("This is some text"),
    ];

    for row in rows {
        check_something(&row);
    }
}


fn check_something(text: &str) {
    static RE: Lazy<Regex> = Lazy::new(|| {
        println!("Compiling regex");
        Regex::new(r"Hello").unwrap()
    });

    let is = RE.is_match(text);
    println!("Is match: {}", is);
}

