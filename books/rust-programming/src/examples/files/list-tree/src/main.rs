use std::env;
use std::fs;
use std::path::Path;
use std::process::exit;

fn main() {
    let path = get_path();

    let tree = list_dir(Path::new(&path));
    for path in tree {
        println!("{path}");
    }
}

fn get_path() -> String {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("More than enough command line parameters");
        exit(1);
    }

    if args.len() == 2 {
        println!("{}", args[1]);
        return args[1].to_owned();
    }

    String::from(".")
}

fn list_dir(path: &Path) -> Vec<String> {
    let mut pathes: Vec<String> = vec![];
    for entry in path.read_dir().expect("read_dir call failed").flatten() {
        //println!("{:?}", entry.path());
        pathes.push(entry.path().to_str().unwrap().to_owned());
        let metadata = fs::metadata(entry.path());
        let file_type = metadata.expect("Could not access file").file_type();
        if file_type.is_dir() {
            pathes.extend(list_dir(&entry.path()));
        }
    }

    pathes
}
