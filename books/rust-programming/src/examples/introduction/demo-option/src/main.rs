#![allow(clippy::needless_late_init)]

fn main() {
    let mut input = None;

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        input = Some(&args[1]);
    }

    match input {
        Some(thing) => {
            println!("The input is {thing}");
        }
        None => {
            println!("There was no input")
        }
    }
}
