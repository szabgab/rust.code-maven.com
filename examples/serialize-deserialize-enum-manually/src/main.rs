#[derive(Debug)]
enum Animal {
    Cat,
    Dog,
    Fish,
}

impl Animal {
    fn new(text: &str) -> Self {
        match text {
            "cat" => Animal::Cat,
            "dog" => Animal::Dog,
            "fish"  => Animal::Fish,

            "hound" => Animal::Dog,

            // default is required
            _ => panic!("No such animal as {text:?}"),
        }
    }
}

impl std::fmt::Display for Animal {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Animal::Cat => "cat",
            Animal::Dog => "dog",
            Animal::Fish => "fish",
        };
        write!(format, "{text}")
    }
}

fn main() {
    let c = Animal::new("cat");
    println!("{c:?}");
    println!("{c}");

    let d = Animal::new("hound");
    println!("{d}");

    let m = Animal::new("mouse");
    println!("{m}");
}

