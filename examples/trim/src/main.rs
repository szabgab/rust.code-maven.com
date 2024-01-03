fn main() {
    let text = "   text with space inside and outside \t \n\n";
    println!("original:   '{}'", text);
    println!("trim:       '{}'", text.trim());
    println!("trim_end    '{}'", text.trim_end());
    println!("trim_start: '{}'", text.trim_start());
}
