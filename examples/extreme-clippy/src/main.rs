#[allow(clippy::print_stdout)]
fn main() {
    println!("Hello, world!");
}

fn echo(text: &str) -> String {
    dbg!(text);
    text.to_owned()
}