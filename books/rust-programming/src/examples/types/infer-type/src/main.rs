fn main() {
    let y: i8 = 42; // explicitely set the type
    println!("{y}");

    let x = 42; // defaults to i32
    println!("{x}");

    let z = 42; // at first Rust will assume this is i32, the default

    let result = y + z; 
    // When Rust sees this it will understand that "z" participates
    // in a operation where both operands have to be the same type.
    // The variable "y", the other operand, was explicitely set to be i8.
    // So Rust infers that z must be also of type i8.
    // It also infers that the variable "result" will be of type i8.

    println!("{result}");
}
