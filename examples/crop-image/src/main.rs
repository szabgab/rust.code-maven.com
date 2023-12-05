// [crop](https://docs.rs/image/latest/image/enum.DynamicImage.html#method.crop)

fn main() {
    let (infile, outfile, x, y, width, height) = get_args();

    let mut img = image::open(infile).unwrap();
    println!("Original width={}, height={}", img.width(), img.height());

    println!(
        "Cropped to: ({},{}) width={}, height={}",
        x, y, width, height
    );

    let cropped = img.crop(x, y, width, height);
    cropped.save(outfile).unwrap();
}

fn get_args() -> (String, String, u32, u32, u32, u32) {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 7 {
        eprintln!("Usage: {} INFILE OUTFILE X Y WIDTH HEIGHT", args[0]);
        std::process::exit(1);
    }
    (
        args[1].to_owned(),
        args[2].to_owned(),
        args[3].parse().unwrap(),
        args[4].parse().unwrap(),
        args[5].parse().unwrap(),
        args[6].parse().unwrap(),
    )
}
