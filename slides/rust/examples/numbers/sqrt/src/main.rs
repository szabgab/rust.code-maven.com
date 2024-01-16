fn main() {
    // let x = 16;
    // println!("{}", x.sqrt());
    // can't call method `sqrt` on ambiguous numeric type `{integer}`
    // you must specify a type for this binding, like `i32`

    let x: i32 = 16;
    //println!("{}", x.sqrt());
    // no method named `sqrt` found for type `i32` in the current scope

    let x_square = (x as f64).sqrt();
    println!("{}", x_square);
}
