use image::imageops::FilterType;

fn main() {
    let (infile, outfile, width, filter) = get_args();

    let img = match image::open(&infile) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Could not read the image file '{}' {}", infile, err);
            std::process::exit(1);
        }
    };

    println!("Original width={}, height={}", img.width(), img.height());

    let height = width * img.height() / img.width();

    println!("Resizing to: width={}, height={}", width, height);

    let scaled = img.resize(width, height, filter);
    match scaled.save(&outfile) {
        Ok(_) => {}
        Err(err) => eprintln!("Could not save file '{}' {}", outfile, err),
    };
}

fn get_args() -> (String, String, u32, FilterType) {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 4 && args.len() != 5 {
        usage(&args[0]);
    }

    let width: u32 = match args[3].parse() {
        Ok(value) => value,
        Err(err) => {
            eprintln!(
                "Invalid parameter for width: '{}'. It must be an integer",
                err
            );
            usage(&args[0])
        }
    };

    let filter_name = if args.len() == 5 { &args[4] } else { "default" };

    (
        args[1].to_owned(),
        args[2].to_owned(),
        width,
        get_filter_type(filter_name),
    )
}

fn get_filter_type(name: &str) -> FilterType {
    match name {
        "nearest" => FilterType::Nearest,
        "triangle" => FilterType::Triangle,
        "cubic" => FilterType::CatmullRom,
        "gauss" => FilterType::Gaussian,
        "lanczos" => FilterType::Lanczos3,
        _ => FilterType::Nearest,
    }
}

fn usage(name: &str) -> ! {
    eprintln!("Usage: {} INFILE OUTFILE WIDTH FILTER", name);
    std::process::exit(1);
}
