use image::{RgbImage, ImageBuffer};

fn main() {
    empty();
}

fn empty() {
    let width = 200;
    let height = 200;

    let img: RgbImage = ImageBuffer::new(width, height);
    img.save("empty.png").unwrap();
}



