fn main() {
    let bd = directories::BaseDirs::new().unwrap();
    let home_dir = bd.home_dir();

    println!("home_dir: {:?}", home_dir);
}
