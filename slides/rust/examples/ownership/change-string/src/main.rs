fn main() {
    let mut text = String::from("Foo");
    println!("Main: {text}");
    display(&text);
    change(&mut text);
    println!("Main: {text}");
    display(&text);
}

fn display(txt: &str) {
    println!("Display: {txt}")
}

fn change(txt: &mut String) {
    //*txt = String::from("Bar");
    txt.push_str(" Bar");
}
