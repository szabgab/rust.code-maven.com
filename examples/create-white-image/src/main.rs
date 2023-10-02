use image::{RgbImage, ImageBuffer};

fn main() {
    white_image();
}

fn white_image() {
    let width = 300;
    let height = 200;

    let mut img: RgbImage = ImageBuffer::new(width, height);
    let red = 255 as u8;
    let green = 255;
    let blue = 255;

    for x in 0..width {
        for y in 0..height {
            *img.get_pixel_mut(x, y) = image::Rgb([red, green, blue]);
        }
    }

    img.save("white.png").unwrap();
}
