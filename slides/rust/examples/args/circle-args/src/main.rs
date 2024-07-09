use std::env;
use std::process::exit;

const PI: f64 = 3.14;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage {} radius", args[0]);
        exit(1);
    }
    let radius = args[1].parse::<f64>().expect("Wanted a number");

    let area = radius * radius * PI;
    let circumference = 2.0 * radius * PI;

    println!("area: {area}");
    println!("circumference: {circumference}");
}
