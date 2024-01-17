macro_rules! s(($result:expr) => ($result.to_string()));

fn main() {
    let name = s!("Foo Bar");
    println!("Hello, {}!", name);
}
