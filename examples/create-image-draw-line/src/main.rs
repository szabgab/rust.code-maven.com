use image::{RgbImage, ImageBuffer};

fn main() {
    red_horizontal_line();
    green_vertical_line();
}

fn red_horizontal_line() {
    let width = 200;
    let height = 200;

    let red = 255 as u8;
    let green = 0;
    let blue = 0;

    let mut img: RgbImage = ImageBuffer::new(width, height);
    let y = 100;
    for x in 20..180 {
        *img.get_pixel_mut(x, y) = image::Rgb([red, green, blue]);
    }

    img.save("red_horizontal_line.png").unwrap();
}

fn green_vertical_line() {
    let width = 200;
    let height = 200;

    let red = 0 as u8;
    let green = 255;
    let blue = 0;

    let mut img: RgbImage = ImageBuffer::new(width, height);
    let x = 100;
    for y in 20..180 {
        *img.get_pixel_mut(x, y) = image::Rgb([red, green, blue]);
    }

    img.save("green_vertical_line.png").unwrap();
}
