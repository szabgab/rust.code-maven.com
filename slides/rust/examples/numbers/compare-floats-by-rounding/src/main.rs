macro_rules! round {
    ($number: expr, $precision: expr) => {
        (10_u32.pow($precision) as f64 * $number).round() / 10_u32.pow($precision) as f64
    };
}

fn main() {
    let x:f64 = 0.1 + 0.2;
    let y = 0.3;
    println!("{x}");
    println!("{y}");
    println!("{}", x == y);
    println!();

    println!("{}", (100.0 * x).round() / 100.0);
    println!("{}", ((100.0 * x).round() / 100.0) == y);

    println!("{}", round64(x, 2));
    println!("{}", round64(x, 2) == y);


    println!("{}", round!(x, 2) == y);
}

fn round64(number:f64, precision:u32) -> f64 {
    (10_u32.pow(precision) as f64 * number).round() / 10_u32.pow(precision) as f64
}


