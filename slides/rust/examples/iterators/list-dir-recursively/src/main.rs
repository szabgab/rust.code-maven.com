use std::path::Path;

fn main() {
    let path = get_dirname();
    let path = Path::new(&path);
    list_dir(path);

}

fn get_dirname() -> String {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} PATH", args[0]);
        std::process::exit(1);
    }

    args[1].to_owned()
}

fn list_dir(path: &Path) {
    println!("path: {:?}", path);
    match path.read_dir() {
        Ok(dh) => {  // A ReadDir instance
            for entry in dh {
                println!("{:?}", entry);
                if let Ok(entry) = entry {
                    if entry.path().is_dir() {
                        println!("DIR");
                        list_dir(&entry.path());
                    }
                }
            }
        },
        Err(err) => println!("Could not open directory '{:?}': {}", path, err),
    }
}

