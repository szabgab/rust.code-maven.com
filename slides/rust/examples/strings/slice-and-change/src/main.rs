fn main() {
    let mut text = String::from("Foobar");
    println!("{}", text);

    let slice = &text[0..=2];
    println!("{}", slice);

    text.clear();
    text.push_str("qwerty");
    println!("{}", text);
    //println!("{}", slice);
}
