use std::ffi::OsStr;
use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("hello.txt");
    println!("{:?} {:?}", path, path.extension().unwrap());

    //let path = PathBuf::from("hello");
    //println!("{:?} {:?}", path, path.extension().unwrap());
    println!();

    for filename in ["hello.txt", "hello", "main.rs"] {
        let path = PathBuf::from(filename);
        match path.extension() {
            Some(ext) => println!("match:     {:?}  {:?}", path, ext),
            None => println!("match:     {:?}  has no extension", path),
        }
        // ext is not in scope here
        //println!("{}", ext);

        let ext = match path.extension() {
            Some(ext) => ext,
            None => OsStr::new(""),
        };
        println!("let match: {:?}  {:?}", path, ext);

        let ext = path.extension().unwrap_or_default();
        println!("default:   {:?}  {:?}", path, ext);

        let Some(ext) = path.extension() else {
            println!("let:       {:?}  has no extension", path);
            continue;
        };
        println!("let:       {:?}  {:?}", path, ext);
    }
}
