macro_rules! ptr {
    ($var: expr) => {
        println!("p: {:p} ptr: {:<15?} len: {:2}, capacity: {:2} '{}'", &$var, $var.as_ptr(), $var.len(), $var.capacity(), $var)        
    };
}

fn main() {
    // Create an empty string
    let mut text = String::new();
    ptr!(text);

    // ASCII table: https://www.ascii-code.com/
    // Unicode table https://home.unicode.org/   and charts https://www.unicode.org/charts/
    // one simple character (from ASCII) is one byte
    text.push('a');
    ptr!(text);
    

    text.push('á');
    ptr!(text);

    text.push('🍓');
    ptr!(text);

    for byte in "🍓".bytes() {
        println!("{byte}");
    }

    let c = String::from_utf8(vec![240, 159, 141, 147]).unwrap();
    ptr!(c);

    let pre = &text[0..3];
    println!("{pre}");

    let pre = &text[0..4];
    println!("{pre}");

}
