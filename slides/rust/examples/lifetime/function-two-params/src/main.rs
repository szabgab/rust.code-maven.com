fn main() {
    let a = String::from("Foo");
    let b = String::from("Bar");
    let choice = true;

    let c = select(&a, &b, choice);    

    println!("{}", c);
    println!("{}", a);    

}


// fn select<>(name1: &str, name2: &str, choice: bool) -> &str {
//     if choice { name1 } else { name2 }
// }

fn select<'a>(name1: &'a str, name2: &'a str, choice: bool) -> &'a str {
    if choice { name1 } else { name2 }
}

