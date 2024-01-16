fn main() {
    let numbers = vec![10, 11, 12];
    println!("{}", numbers.len());

    // println!("{}", numbers);
    // `Vec<{integer}>` doesn't implement `std::fmt::Display`
    // `Vec<{integer}>` cannot be formatted with the default formatter

    println!("{:?}", numbers);
    println!("{:#?}", numbers);
}
