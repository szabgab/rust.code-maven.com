use std::cmp::Ordering;
fn main() {
    let x = 23;
    let y = 32;
    match x.cmp(&y) {
        Ordering::Greater => println!("x is bigger than y"),
        Ordering::Less => println!("x is smaller than y"),
        Ordering::Equal => println!("x is equal to y"),
    }

    println!(
        "{}",
        match x.cmp(&y) {
            Ordering::Greater => "x is bigger than y",
            Ordering::Less => "x is smaller than y",
            Ordering::Equal => "x is equal to y",
        }
    )
}
