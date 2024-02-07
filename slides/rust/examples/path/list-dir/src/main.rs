fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} PATH", &args[0]);
        std::process::exit(1);
    }
    let path = std::path::PathBuf::from(&args[1]);

    let filenames = path
        .read_dir()
        .unwrap()
        .map(|de| de.unwrap().file_name().to_str().unwrap().to_owned())
        .collect::<Vec<String>>();

    println!("{:?}", filenames);
    println!();

    let relative_pathes = path
        .read_dir()
        .unwrap()
        .map(|de| de.unwrap().path().as_os_str().to_str().unwrap().to_owned())
        .collect::<Vec<String>>();

    println!("{:?}", relative_pathes);
    println!();

    let absolute_pathes = path
    .read_dir()
    .unwrap()
    .map(|de| de.unwrap().path().canonicalize().unwrap().as_os_str().to_str().unwrap().to_owned())
    .collect::<Vec<String>>();

    println!("{:?}", absolute_pathes);
    
 
}
