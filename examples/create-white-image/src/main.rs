use image::{ImageBuffer, RgbImage};

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

    for x in 0..width {
        *img.get_pixel_mut(x, 0) = image::Rgb([0, 0, 0]);
        *img.get_pixel_mut(x, height - 1) = image::Rgb([0, 0, 0]);
    }
    for y in 0..height {
        *img.get_pixel_mut(0, y) = image::Rgb([0, 0, 0]);
        *img.get_pixel_mut(width - 1, y) = image::Rgb([0, 0, 0]);
    }

    img.save("white.png").unwrap();
}
