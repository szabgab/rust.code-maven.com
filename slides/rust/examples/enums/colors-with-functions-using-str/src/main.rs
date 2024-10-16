#[derive(Debug, PartialEq)]
#[allow(dead_code)]
enum Color<'a> {
    Red,
    Green,
    Blue,
    White,
    Black,
    Other(&'a str),
}

fn to_rgb<'a>(color: &'a Color) -> &'a str {
    match color {
        Color::Black => "000000",
        Color::Red => "ff0000",
        Color::Green => "00ff00",
        Color::Blue => "0000ff",
        Color::White => "ffffff",
        &Color::Other(val) => val,
    }
}


fn from_rgb(rgb: &str) -> Color {
    match rgb {
        "000000" => Color::Black,
        "ff0000" => Color::Red,
        "00ff00" => Color::Green,
        "0000ff" => Color::Blue,
        "ffffff" => Color::White,
        val => Color::Other(val),
    }
}


fn main() {
    let background = Color::Black;
    let foreground = Color::White;
    let ink = Color::Black;
    let sky = Color::Blue;
    let other = Color::Other("4674b9");

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
    assert_eq!(to_rgb(&sky), "0000ff");
    assert_eq!(to_rgb(&other), "4674b9");
}
