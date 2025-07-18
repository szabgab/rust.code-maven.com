fn main() {
    let mut num: i8 = 126;
    println!("{num}");

    num = num.checked_add(1).unwrap_or(42);
    println!("{num}");

    num = num.checked_add(1).unwrap_or(42);
    println!("{num}");
}
