fn main() {
    println!("Hello, second");

    let x = "19";
    let x = x.parse::<u32>().unwrap();
    println!("{x}");
}

#[test]
fn check_second() {
    assert_eq!(1, 1);
}
