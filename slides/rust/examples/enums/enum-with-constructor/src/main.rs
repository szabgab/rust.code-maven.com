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
    fn from_rgb(rgb: &str) -> Self {
        return match rgb.to_ascii_lowercase().as_str() {
            "000000" => Color::Black,
            "ff0000" => Color::Red,
            "00ff00" => Color::Green,
            "0000ff" => Color::Blue,
            "ffffff" => Color::White,
            _ => panic!("Unhandled color {rgb}"),
        }
    }
}

fn main() {
    let background = Color::from_rgb("000000");
    let foreground = Color::from_rgb("ffffff");
    let ink = Color::from_rgb("000000");
    let frame = Color::from_rgb("FF0000");

    for color in [background, foreground, ink, frame] {
        println!("{:?}", color);
    }
}
