use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let filename = get_filename();
    match File::open(&filename) {
        Ok(mut file) => {
            let mut content = String::new();

            match file.read_to_string(&mut content) {
                Ok(size) => {
                    println!("Read {size} bytes.");
                    println!("We have a string of {} bytes.", content.len());
                    println!("{content}");
                }
                Err(err) => eprintln!("Error: {err}"),
            }
        }
        Err(error) => {
            eprintln!("Error opening file {filename}: {error}");
        }
    }
}

fn get_filename() -> String {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        eprintln!("Usage: {} data.txt", args[0])
    }

    args[1].to_owned()
}
