#[allow(dead_code)]
enum ColorName {
    Red(String),
    Green(String),
    Blue(String),
    White(String),
    Black(String),
}


fn main() {
    let background = ColorName::Black(String::from("#000000"));
    let foreground = ColorName::White(String::from("#FFFFFF"));
    let ink = ColorName::Black(String::from("#FFFFFF"));
    let frame = ColorName::Red(String::from("#FF0000"));

    for color in [background, foreground, ink, frame] {
        match color {
            ColorName::White(val) => println!("white: {val}"),
            ColorName::Red(_val) => println!("red:"),
            _ => println!("other"),
        }
    }
}
