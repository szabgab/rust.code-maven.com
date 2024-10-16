fn main() {
    let text = String::from("The black cat: 🐈‍ climbed the green tree: 🌳!");
    println!("{}", text);
    println!("'{}'", &text[4..4]); // ''  empty string
    println!("'{}'", &text[4..=4]); // 'b'
    println!("'{}'", &text[4..9]); // 'black'
    println!("'{}'", &text[30..]); // ' the green tree: 🌳!'
    println!("'{}'", &text[..4]); // 'The '

    println!("'{}'", &text[15..22]); // '🐈‍'

    //println!("'{}'", &text[16..22]); // panic!: byte index 16 is not a char boundary; it is inside '🐈' (bytes 15..19)

    //println!("'{}'", &text[25..60]);  // panic! at 'byte index 60 is out of bounds
}
