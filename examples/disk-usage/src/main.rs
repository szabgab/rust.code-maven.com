use walkdir::WalkDir;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} PATH", args[0]);
        std::process::exit(1);
    }
    let root = &args[1];

    let size = du(root);
    println!("{root} size: {size}");

}

fn du(root: &str) -> u128 {
    let mut size = 0;
    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();
        if entry.path().is_file() {
            if let Ok(meta) = entry.path().metadata() {
                size += meta.len() as u128;
            }
        }
    }

    size
}
