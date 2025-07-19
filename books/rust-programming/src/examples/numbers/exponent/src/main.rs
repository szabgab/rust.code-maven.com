fn main() {
    // let x  = 3;
    // println!("{}", x.pow(2));
    // can't call method `pow` on ambiguous numeric type `{integer}`
    // you must specify a type for this binding, like `i32`

    let x: i32 = 3;
    println!("{}", x.pow(2));

    let y: i8 = 10;
    println!("{}", y.pow(2));

    let lucky_number = get_lucky_number();
    println!("{}", lucky_number.pow(2));

    // let y: i8 = 16;
    // println!("{}", y.pow(2));
    // panic!: attempt to multiply with overflow
}

fn get_lucky_number() -> i16 {
    23
}
