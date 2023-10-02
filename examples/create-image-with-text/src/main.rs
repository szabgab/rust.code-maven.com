use image::{Rgb, RgbImage};
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::env;
use std::path::Path;

fn main() {
    let argv = env::args().collect::<Vec<String>>();
    if argv.len() != 3 {
        eprintln!("Usage: {} FILENAME TEXT", &argv[0]);
        std::process::exit(1);
    }
    let path = Path::new(&argv[1]);
    let text = &argv[2];

    let width = 400;
    let height = 200;

    // create image
    let mut image = RgbImage::new(width, height);
    // set white background
    for x in 0..width {
        for y in 0..height {
            *image.get_pixel_mut(x, y) = image::Rgb([255, 255, 255]);
        }
    }

    //let font = Vec::from(include_bytes!("/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf") as &[u8]);
    //let font = Vec::from(include_bytes!("/snap/cups/980/usr/share/fonts/truetype/freefont/FreeSans.ttf") as &[u8]);
    let font = Vec::from(include_bytes!("../FreeSans.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let inteded_text_height = 24.4;
    let scale = Scale {
        x: inteded_text_height * 2.0,
        y: inteded_text_height,
    };

    // color of the text
    let red = 50 as u8;
    let green = 50;
    let blue = 0;

    // get the size of the text and calculate the x, y coordinate where to start to be center aligned
    // both horizontally and vertically
    let (text_width, text_height) = text_size(scale, &font, text);
    println!("Text size: {}x{}", text_width, text_height);
    let text_start_x = ((width-text_width as u32) / 2 ) as i32;
    let text_start_y = ((height-text_height as u32) / 2 ) as i32;

    draw_text_mut(&mut image, Rgb([red, green, blue]), text_start_x, text_start_y, scale, &font, text);

    image.save(path).unwrap();
}