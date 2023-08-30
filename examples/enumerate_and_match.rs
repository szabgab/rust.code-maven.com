#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let background = Color::Blue;
    let foreground = Color::Red;
    println!("{:?}", background);
    println!("{:?}", foreground);

    println!("is background red?  {}", background == Color::Red);
    println!("is background blue? {}", background == Color::Blue);

    for clr in [foreground, background] {
        match clr {
            Color::Red => println!("red"),
            Color::Green => println!("green"),
            Color::Blue => println!("blue"),
        }
    }
}
