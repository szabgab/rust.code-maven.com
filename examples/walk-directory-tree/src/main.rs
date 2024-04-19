use walkdir::{DirEntry, WalkDir};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} PATH", args[0]);
        std::process::exit(1);
    }
    let root = &args[1];

    // println!("{root}");

    // for entry in WalkDir::new(root) {
    //     let entry = entry.unwrap();
    //     println!("{}", entry.path().display());
    // }

    for entry in WalkDir::new(root).into_iter().filter_entry(skip_things) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }
}

fn skip_things(entry: &DirEntry) -> bool {
    !entry
        .file_name()
        .to_str()
        .map(|name| entry.path().is_dir() && name == "target")
        .unwrap_or(false)
}
