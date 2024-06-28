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

            "cica" => Animal::Cat,

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
    let a = Animal::new("cica");
    println!("{a:?}");
    println!("{a}");
    //let a = Animal::new("mouse");

    // with_if();
    // with_match();
    // with_enum();

    // let a = Animal::Cat;
    // println!("{a:?}");

    //let x = Animal::from("cat");

}

// fn with_enum() {
//     let args = std::env::args().collect::<Vec<String>>();

//     if args.len() != 2 {
//         eprintln!("Usage: {} [cat|dog|fish]", args[0]);
//         std::process::exit(1);        
//     }
//     let animal = &args[1];
//     let color_variant = Animal::from_str(&args[1]).unwrap();

// }

// fn with_if() {
//     let args = std::env::args().collect::<Vec<String>>();
//     if args.len() != 2 {
//         eprintln!("Usage: {} [cat|dog|fish]", args[0]);
//         std::process::exit(1);        
//     }
//     let animal = &args[1];

//     if animal == "cat" {
//         println!("meow");
//     } else if animal == "dog" {
//         println!("woof");
//     } else if animal == "fish" {
//         println!("");
//     } else { // default is optional
//         println!("No such animal as {animal:?}")
//     }
// }


// fn with_match() {
//     let args = std::env::args().collect::<Vec<String>>();
//     if args.len() != 2 {
//         eprintln!("Usage: {} [cat|dog|fish]", args[0]);
//         std::process::exit(1);        
//     }
//     let animal = args[1].as_str();

//     match animal {
//         "cat" => println!("meow"),
//         "dog" => println!("woof"),
//         "fish"  => println!(""),

//         // default is required
//         _ =>println!("No such animal as {animal:?}"),
//     };
// }
