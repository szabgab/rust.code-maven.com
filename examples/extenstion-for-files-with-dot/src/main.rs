use std::path::PathBuf;
fn main() {
    let path = PathBuf::from("code.rs");
    println!("{:?} {:?}", path, path.extension().unwrap());

    let path = PathBuf::from("my-data-1.70.html");
    println!("{:?} {:?}", path, path.extension().unwrap());
}
