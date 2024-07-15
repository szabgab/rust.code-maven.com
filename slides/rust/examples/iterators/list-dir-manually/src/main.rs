use std::path::Path;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} PATH", args[0]);
        std::process::exit(1);
    }

    list_dir(&args[1]);
}

fn list_dir(path_from_user: &str) {
    let path = Path::new(path_from_user);

    match path.read_dir() {
        Ok(mut dh) => {
            // A ReadDir instance
            loop {
                let entry = dh.next();
                match entry {
                    Some(value) => println!("{:?}", value),
                    None => break,
                }
            }
        }
        Err(err) => println!("Could not open directory '{}': {}", path_from_user, err),
    }
}
