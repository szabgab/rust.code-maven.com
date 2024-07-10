fn main() {
    let x = 42; // defaults to i32
    println!("{x}");

    let y: i8 = 42; // explicitely set the type
    let z = 42; // at first Rust will assume this is i32, the defaul

    let result = y + z; // When Rust sees this it will understand that z participates
                        // in a operstion where both operands have to be the same type and the other operand
                        // was explicitely set to be i8, so Rust infers that z is also of type i8.
                        // It also infers that result will be of type i8

    println!("{result}");
}
