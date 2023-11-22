use std::fs::read_to_string;

fn main() {
    let filename = "data.txt";
    let content = read_to_string(filename).unwrap();
    println!("------");
    print!("{}", content);
    println!("------");
}
