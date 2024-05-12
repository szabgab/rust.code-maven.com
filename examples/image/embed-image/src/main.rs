use image::{GenericImageView, ImageBuffer, RgbaImage};

fn main() {
    let mut img = create_image();


    let infile = "rust.png";
    let logo = image::open(infile).unwrap();

    println!("Original width={}, height={}", logo.width(), logo.height());
    let start_x = 100;
    let start_y = 50;

    println!("New image: width={}, height={}", img.width(), img.height());
    if start_x + logo.width() > img.width() {
        println!("Does not fit in width");
        return;
    }
    if start_y + logo.height() > img.height() {
        println!("Does not fit in height");
        return;
    }

    for x in 0..logo.width() {
        for y in 0..logo.height() {
            *img.get_pixel_mut(start_x + x, start_y + y) = logo.get_pixel(x, y);
        }
    }

    img.save("created.png").unwrap();

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

    // for x in 0..width {
    //     *img.get_pixel_mut(x, 0) = image::Rgb([0, 0, 0]);
    //     *img.get_pixel_mut(x, height - 1) = image::Rgb([0, 0, 0]);
    // }
    // for y in 0..height {
    //     *img.get_pixel_mut(0, y) = image::Rgb([0, 0, 0]);
    //     *img.get_pixel_mut(width - 1, y) = image::Rgb([0, 0, 0]);
    // }

    img
    
}
