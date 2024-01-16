fn main() {
    let x: i8 = 10;
    let y: i8 = 16;

    let z = x.pow(2);
    println!("{}", z); // 100

    // let z = y.pow(2);
    // panic: attempt to multiply with overflow

    let z = x.checked_pow(2).unwrap();
    println!("{}", z); // 100

    // let z = y.checked_pow(2).unwrap();
    // panic: called `Option::unwrap()` on a `None` value

    let z = y.checked_pow(2).unwrap_or(0);
    println!("{}", z); // 0

    let z = match y.checked_pow(2) {
        Some(val) => val,
        None => {
            eprintln!("overflow");
            std::process::exit(1);
        }
    };
    println!("{}", z); // isn't reached if there is an overflow
}
