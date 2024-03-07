fn main() {
    println!("Hello, main");

    let x = "23";
    let x = x.parse::<u32>().unwrap();
    println!("{x}");
}

#[test]
fn check_main() {
    assert_eq!(1, 1);
}
