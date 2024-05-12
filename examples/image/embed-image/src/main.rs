use image::{GenericImageView, ImageBuffer, RgbaImage};

fn main() {
    let img = create_image();

    let img = embed_image(img, "rust.png", 100, 50);

    img.save("created.png").unwrap();

}

fn embed_image(mut img: RgbaImage, infile: &str, start_x: u32, start_y: u32) -> RgbaImage {
    let logo = image::open(infile).unwrap();

    println!("Base image: width={}, height={}", img.width(), img.height());
    println!("Embedding:  width={}, height={}", logo.width(), logo.height());

    if start_x + logo.width() > img.width() {
        println!("Does not fit in width");
        return img;
    }
    if start_y + logo.height() > img.height() {
        println!("Does not fit in height");
        return img;
    }

    for x in 0..logo.width() {
        for y in 0..logo.height() {
            *img.get_pixel_mut(start_x + x, start_y + y) = logo.get_pixel(x, y);
        }
    }

    img
}

fn create_image() -> RgbaImage {
    let width = 300;
    let height = 200;

    let mut img: RgbaImage = ImageBuffer::new(width, height);
    let red = 0 as u8;
    let green = 255;
    let blue = 255;
    let alpha = 255;

    for x in 0..width {
        for y in 0..height {
            *img.get_pixel_mut(x, y) = image::Rgba([red, green, blue, alpha]);
        }
    }

    img
    
}
