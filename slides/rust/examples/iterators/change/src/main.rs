fn main() {
    let mut numbers = [2, 3, 4];
    println!("{numbers:?}");

    for num in numbers.iter_mut() {
        *num += 1;
    }

    println!("{numbers:?}");
}
