#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
    Other(String),
}

impl Color {
    fn to_rgb(&self) -> &str {
        return match self {
            Color::Black => "000000",
            Color::Red => "ff0000",
            Color::Green => "00ff00",
            Color::Blue => "0000ff",
            Color::White => "ffffff",
            Color::Other(val) => val.as_str(),
        };
    }
}

fn main() {
    let background = Color::Black;
    let foreground = Color::White;
    let ink = Color::Black;
    let sky = Color::Blue;
    let other = Color::Other(String::from("4674b9"));

    for color in [background, foreground, ink, sky, other] {
        println!("{:?}  {}", color, color.to_rgb());
    }
}
