fn main() {
    let names = [
        String::from("name.txt"),
        String::from("other"),
        String::from("narnia"),
    ];
    for name in names {
        print!("name: '{name}'");
        if name.starts_with("na") {
            print!(" starts with na");
        }
        println!();
    }
}
