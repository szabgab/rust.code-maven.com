fn main() {
    let a = 7;
    let b = 8;
    println!("{}", add32(a, b));

    let c = 7_i8;
    let d = 29_i8;
    println!("{}", add8(c, d));

    println!("{}", add(a, b));
    println!("{}", add(c, d));
}

fn add32(x: i32, y: i32) -> i32 {
    x + y
}

fn add8(x: i8, y: i8) -> i8 {
    x + y
}

fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
    print_type(&x);
    x + y
}

fn print_type<T>(_: &T) {
    println!("{:?}", std::any::type_name::<T>());
}
