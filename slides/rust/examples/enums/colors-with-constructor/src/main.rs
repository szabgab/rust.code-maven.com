#[derive(Debug)]
#[allow(dead_code)]
enum Color<'a> {
    Red,
    Green,
    Blue,
    White,
    Black,
    Other(&'a str)
}

impl<'a> Color<'a> {
    fn from_rgb(rgb: &'a str) -> Self {
        return match rgb {
            "000000" => Color::Black,
            "ff0000" => Color::Red,
            "00ff00" => Color::Green,
            "0000ff" => Color::Blue,
            "ffffff" => Color::White,
            val => Color::Other(val),
            //_ => panic!("Unhandled color {rgb}"),
            
        }
    }
}

fn main() {
    let background = Color::from_rgb("000000");
    let foreground = Color::from_rgb("ffffff");
    let ink = Color::from_rgb("000000");
    let sky = Color::Blue;
    let other = Color::Other("4674b9");


    for color in [background, foreground, ink, sky, other] {
        println!("{:?}", color);
    }
}
