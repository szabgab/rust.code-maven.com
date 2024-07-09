// Experiment to use a function accepting strings without explaining the references
// to be used at the beginning of the course


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    println!("{}", add(args[1].clone(), args[2].clone()));
}

fn add(x: String, y: String) -> u32 {
    let a: u32  = x.parse().expect("number");
    let b: u32  = y.parse().expect("number");
    a + b
}
