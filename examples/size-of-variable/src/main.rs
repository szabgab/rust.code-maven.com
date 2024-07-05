use std::mem;

macro_rules! prt_string {
    ($name: expr, $var: expr) => {
        println!("{:15}: {:3} {:2} {:2}", $name, mem::size_of_val(&$var), $var.len(), $var.capacity());        
    };
}

#![feature(layout_for_ptr)]
fn main() {
    let number_i8: i8 = 5;
    println!("i8: {}", mem::size_of_val(&number_i8));

    let number_i16: i16 = 5;
    println!("i16: {}", mem::size_of_val(&number_i16));

    let number_i32 = 5;
    println!("i32: {}", mem::size_of_val(&number_i32));

    let number_i64: i64 = 5;
    println!("i64: {}", mem::size_of_val(&number_i64));

    let number_u64: u64 = 5;
    println!("u64: {}", mem::size_of_val(&number_u64));

    let number_f32: f32 = 5.0;
    println!("f32: {}", mem::size_of_val(&number_f32));

    let empty_string = String::new();
    prt_string!("empty string", empty_string);

    let string_10 = String::from("0123456789");
    prt_string!("string_10", string_10);

    let string_50 = String::from("01234567890123456789012345678901234567890123456789");
    prt_string!("string_50", string_50);

    let name: Vec<u32> = vec![];
    prt_string!("name", name);
    //mem::size_of_val_raw
    println!("{}", mem::size_of_val_raw(&string_50));

}


