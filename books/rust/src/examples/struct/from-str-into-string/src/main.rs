fn main() {
    let a = "Hello World!";
    println!("{a}");

    let b = String::from(a);
    println!("{b}");

    let c: String = a.into();
    println!("{c}");

    // let d = a.into::<String>();
    // method takes 0 generic arguments but 1 generic argument was supplied
    // println!("{d}");

    let e = Into::<String>::into(a);
    println!("{e}");
}
