use image::imageops::FilterType;

fn main() {
    let (infile, outfile, width, filter) = get_args();

    let img = image::open(infile).unwrap();
    println!("Original width={}, height={}", img.width(), img.height());

    let height = width * img.height() / img.width();

    println!("Scaled: width={}, height={}", width, height);

    let scaled = img.resize(width, height, filter);
    scaled.save(outfile).unwrap();
}

fn get_args() -> (String, String, u32, FilterType) {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 5 {
        eprintln!("Usage: {} INFILE OUTFILE WIDTH FILTER", args[0]);
        std::process::exit(1);
    }
    (
        args[1].to_owned(),
        args[2].to_owned(),
        args[3].parse().unwrap(),
        get_filer_type(&args[4]),
    )
}

fn get_filer_type(name: &str) -> FilterType {
    match name {
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "cubic" => FilterType::CatmullRom,
        "gauss" => FilterType::Gaussian,
        "lanczos" => FilterType::Lanczos3,
        _ => FilterType::Nearest,
    }
}
