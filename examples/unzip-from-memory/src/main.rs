use std::{error::Error, fs::create_dir_all, path::PathBuf};

use flate2::bufread::GzDecoder;
use tar::Archive;

fn main() {
    let folder = PathBuf::from("data");
    let zipped = include_bytes!("../example.tar.gz");

    if !folder.exists() {
        create_dir_all(&folder).unwrap()
    }

    unzip(zipped, folder).unwrap();
}

fn unzip(tar_gz: &[u8], folder: PathBuf) -> Result<(), Box<dyn Error>> {
    println!("Unzipping to {folder:?}");

    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(folder)?;

    Ok(())
}
