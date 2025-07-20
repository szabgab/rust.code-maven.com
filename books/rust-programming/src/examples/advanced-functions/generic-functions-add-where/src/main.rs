fn main() {
    let a = 7;
    let b = 8;

    let c = 7_i8;
    let d = 29_i8;

    println!("{}", add(a, b));
    println!("{}", add(c, d));
}

fn add<T>(x: T, y: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    x + y
}
