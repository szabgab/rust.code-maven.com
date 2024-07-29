macro_rules! s {
    ($text:expr) => {
        String::from($text)
    }
}

fn main() {
    let name = s!("Sloppy Joe");
    println!("{name}");
    assert_eq!(name, String::from("Sloppy Joe"));
}
