use std::env;
use std::process::exit;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        println!("Usage: {} x y", args[0]);
        exit(1);
    }
    let x: i32 = args[1].parse().expect("Wanted a number");
    let y: i32 = args[2].parse().expect("Wanted a number");

    println!("{x} + {y} = {}", x+y);
    println!("{x} * {y} = {}", x*y);
    println!("{x} - {y} = {}", x-y);
    println!("{x} / {y} = {}", x/y);
}
