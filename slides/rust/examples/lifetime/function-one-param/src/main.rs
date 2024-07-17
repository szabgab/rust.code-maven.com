fn main() {
    let a = String::from("Foo");
    let choice = true;

    let c = select(&a, choice);    

    println!("{}", c);
    println!("{}", a);    

}


fn select<>(name1: &str, choice: bool) -> &str {
    if choice { name1 } else { name1 }
}


