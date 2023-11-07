pub const ANIMALS: &[&str] = &include!(concat!(env!("OUT_DIR"), "/animals.rs"));

fn main() {
    println!("{:?}", ANIMALS);
    for animal in ANIMALS {
        println!("{}", animal);
    }
}
