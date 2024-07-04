#[derive(Debug)]
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
}

impl Color {
    fn rgb(&self) -> &str {
        return match self {
            Color::Black => "000000",
            Color::Red => "ff0000",
            Color::Green => "00ff00",
            Color::Blue => "0000ff",
            Color::White => "ffffff",
        }
    }
}

fn main() {
    let background = Color::Black;
    let foreground = Color::White;
    let ink = Color::Black;
    let frame = Color::Red;

    for color in [background, foreground, ink, frame] {
        println!("{:?}  {}", color, color.rgb());
    }
}
