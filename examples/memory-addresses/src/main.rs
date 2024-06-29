macro_rules! prt_number {
    ($var:expr) => {
        println!(
            "{:2} {:2} {:p} {:15?} '{}'",
            "",
            "",
            &$var,
            "",
            $var
        );
    };
}

macro_rules! prt_with_capacity {
    ($var:expr) => {
        println!(
            "{:2} {:2} {:p} {:15?} '{}'",
            $var.len(),
            $var.capacity(),
            &$var,
            $var.as_ptr(),
            $var
        );
    };
}

macro_rules! prt_without_capacity {
    ($var:expr) => {
        println!(
            "{:2} {:2} {:p} {:15?} '{}'",
            $var.len(),
            "",
            &$var,
            $var.as_ptr(),
            $var
        );
    };
}

fn main() {
    let embedded_str_1 = "Hello";
    prt_without_capacity!(embedded_str_1);
    let embedded_str_2 = "World";
    prt_without_capacity!(embedded_str_2);

    let integer_u8_1 = 23_u8;
    prt_number!(integer_u8_1);


    let immutable_string = String::from("Hello");
    prt_with_capacity!(immutable_string);


    let mut mutable_string = String::new();
    prt_with_capacity!(mutable_string);
    mutable_string.push_str("Hello");
    prt_with_capacity!(mutable_string);


    let boxed_string = Box::new("Hello");
    prt_without_capacity!(boxed_string);
    println!("{}", boxed_string);
}
