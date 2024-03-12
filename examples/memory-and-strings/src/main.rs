fn main() {
    let mut text = String::new();
    println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &text, text.as_ptr(), text.len(), text.capacity(), text);

    text.push_str("Hello foo!");
    println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &text, text.as_ptr(), text.len(), text.capacity(), text);

    text.replace_range(6..9, "bar");
    println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &text, text.as_ptr(), text.len(), text.capacity(), text);

    let temp = String::from("The black cat");
    println!("{temp}");
    println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &temp, temp.as_ptr(), temp.len(), temp.capacity(), temp);

    text.replace_range(6..9, "qqrq");
    println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &text, text.as_ptr(), text.len(), text.capacity(), text);

    text.replace_range(6..9, "123456");
    println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &text, text.as_ptr(), text.len(), text.capacity(), text);

    text.replace_range(6..9, "12345678901234567890");
    println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &text, text.as_ptr(), text.len(), text.capacity(), text);

}
