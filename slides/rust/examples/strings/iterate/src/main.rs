fn main() {
    let text = String::from("The black cat");
    println!("{text}");
    println!("{:?}", text.chars());

    for ch in text.chars() {
        println!("{ch}");
    }
}
