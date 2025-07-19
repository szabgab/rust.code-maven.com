#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    White,
    Black,
    Other(String),
}

fn to_rgb(color: &Color) -> String {
    match color {
        Color::Black => String::from("000000"),
        Color::Red => String::from("ff0000"),
        Color::Green => String::from("00ff00"),
        Color::Blue => String::from("0000ff"),
        Color::White => String::from("ffffff"),
        Color::Other(val) => val.clone(),
    }
}

fn from_rgb(rgb: &str) -> Color {
    match rgb.to_ascii_lowercase().as_str() {
        "000000" => Color::Black,
        "ff0000" => Color::Red,
        "00ff00" => Color::Green,
        "0000ff" => Color::Blue,
        "ffffff" => Color::White,
        val => Color::Other(val.to_owned()),
    }
}

fn main() {
    let background = Color::Black;
    let foreground = Color::White;
    let ink = Color::Black;
    let sky = Color::Blue;
    let other = Color::Other(String::from("4674b9"));

    let experiment = from_rgb("ab89e2");

    println!("{}", background == foreground);
    assert_ne!(background, foreground);
    println!("{}", background == ink);
    assert_eq!(background, ink);
    println!();

    for color in [&background, &foreground, &ink, &sky, &other, &experiment] {
        println!("{} {:?}", to_rgb(color), color);
    }

    assert_eq!(to_rgb(&background), "000000");
    assert_eq!(to_rgb(&other), "4674b9");
}
