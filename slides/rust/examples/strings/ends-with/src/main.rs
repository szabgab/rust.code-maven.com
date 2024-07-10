fn main() {
    let names = [String::from("name.txt"), String::from("other")];
    for name in names {
        print!("name: '{name}'");
        if name.ends_with(".txt") {
            print!(" ends with .txt");
        }
        println!();
    }
}
