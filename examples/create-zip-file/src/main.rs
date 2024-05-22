use std::{error::Error, path::PathBuf};

use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Builder;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} FILENAME FOLDER", args[0]);
        std::process::exit(1);
    }
    let filename = PathBuf::from(&args[1]);
    let folder = PathBuf::from(&args[2]);
    if filename.exists() {
        eprintln!("file {filename:?}  already exists");
        std::process::exit(1);
    }

    if folder.exists() {
        if !folder.is_dir() {
            eprint!("The second parameter {folder:?} is not a folder");
            std::process::exit(1);
        }
    }

    // check that the folder is not empty

    zip(filename, folder).unwrap();
}

fn zip(filename: PathBuf, folder: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Zipping {folder:?} to {filename:?}");

    let tar_gz = std::fs::File::create(filename)?;
    let encoder = GzEncoder::new(tar_gz, Compression::default());

    let mut archive = Builder::new(encoder);
    archive.append_dir_all(".", folder)?;

    Ok(())
}
