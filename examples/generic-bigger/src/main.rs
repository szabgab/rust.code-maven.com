fn main() {
    let x: i8 = 25;
    let y: i8 = 42;
    println!("i8:  {}", bigger(x, y));

    let x: i16 = 25;
    let y: i16 = 42;
    println!("i16: {}", bigger(x, y));

    let x = 25; // defaults to i32
    let y = 42; // defaults to i32
    println!("i32: {}", bigger(x, y));

    let x: i64 = 25;
    let y: i64 = 42;
    println!("i64: {}", bigger(x, y));

    let x: f32 = 25.0;
    let y: f32 = 42.0;
    println!("f32: {}", bigger(x, y));

    let x = 25.0;
    let y = 42.0;
    println!("f64: {}", bigger(x, y));
}

fn bigger<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}
