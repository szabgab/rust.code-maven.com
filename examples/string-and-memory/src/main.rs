macro_rules! ptr {
    ($var: expr) => {
        println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &$var, $var.as_ptr(), $var.len(), $var.capacity(), $var)        
    };
}

fn main() {
    // Create an empty string
    let mut text = String::new();
    ptr!(text);
    text.push('a');
    ptr!(text);

    text.push('b');
    ptr!(text);

    text.push_str("cdefgh");
    ptr!(text);

    let other = String::from("qqrq");
    ptr!(other);

    text.push('i');
    ptr!(text);

    text.push_str("12345678901234567890");
    ptr!(text);

}

