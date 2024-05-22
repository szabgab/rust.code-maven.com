use std::{error::Error, fs::create_dir_all, path::PathBuf};

use flate2::read::GzDecoder;
use std::{fs, io};
use tar::Archive;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        eprintln!("Usage: {} FILENAME FOLDER", args[0]);
        std::process::exit(1);
    }
    let filename = PathBuf::from(&args[1]);
    let folder = PathBuf::from(&args[2]);
    if !filename.exists() {
        eprintln!("file {filename:?} does not exist");
        std::process::exit(1);
    }

    if folder.exists() {
        if !folder.is_dir() {
            eprint!("The second parameter {folder:?} is not a folder");
            std::process::exit(1);
        }
        
    }
 
    if !folder.exists() {
        create_dir_all(&folder).unwrap()
    }

   // check that the folder is empty

    unzip(filename, folder).unwrap();
}

fn unzip(filename: PathBuf, folder: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Unzipping {filename:?} to {folder:?}");

    let tar_gz = std::fs::File::open(filename)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(folder)?;

    Ok(())
}
