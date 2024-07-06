#[allow(dead_code)]
enum ColorName {
    Red,
    Green,
    Blue,
    White,
    Black,
}

struct Color {
    name: ColorName,
    rgb: String,
}


fn main() {
    let background = Color {
        name: ColorName::Black,
        rgb : String::from("#000000"),
    };
    let foreground = Color {
        name: ColorName::White,
        rgb: String::from("#FFFFFF"),
    };
    let ink = Color {
        name: ColorName::Black,
        rgb: String::from("#FFFFFF"),
    };
    let frame = Color {
        name: ColorName::Red,
        rgb: String::from("#FF0000"),
    };

    for color in [background, foreground, ink, frame] {
        match color.name {
            ColorName::White => println!("white: {}", color.rgb),
            ColorName::Red => println!("red: {}", color.rgb),
            _ => println!("other"),
        }
    }
}
