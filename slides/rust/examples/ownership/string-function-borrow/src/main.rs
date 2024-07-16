fn main() {
    let name = String::from("Foo Bar");
    println!("{name}");
    greet(&name);
    println!("{name}");
}

fn greet(text: &String) {
    println!("Greet: {}", *text); // explicit derefernce
    println!("Greet: {}", text);  // automatic dereference
    println!("Greet: {text}");
}

