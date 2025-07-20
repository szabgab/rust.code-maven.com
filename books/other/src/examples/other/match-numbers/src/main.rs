fn main() {
    let age = 65;
    match age {
        x if x < 18 => println!("You are a minor"),
        x if x >= 18 && x <= 65 => println!("You are an adult"),
        x if x>65 => println!("You are a senior citizen"),
        _ => println!("Invalid age")
    }
}
