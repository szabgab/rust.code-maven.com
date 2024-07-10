fn main() {
    let mut num: i8 = 126;
    println!("{num}");

    num = num.saturating_add(1);
    println!("{num}");

    num = num.saturating_add(1);
    println!("{num}");
}
