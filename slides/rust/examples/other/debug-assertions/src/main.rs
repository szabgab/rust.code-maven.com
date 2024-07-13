
fn main() {
    println!("Hello, world!");
    if cfg!(debug_assertions) {
        println!("debug_assertions");
    }
}
