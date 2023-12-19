fn main() {
    let x: i8 = 25;
    let y: i8 = 42;
    println!("{}", bigger(x, y));

    let x: i16 = 25;
    let y: i16 = 42;
    println!("{}", bigger(x, y));
}

fn bigger<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
