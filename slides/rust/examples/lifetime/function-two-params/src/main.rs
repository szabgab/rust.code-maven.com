fn main() {
    let mut a = String::new();
    let mut b = String::new();
    println!("a {a}");
    println!("b {b}");

    a = String::from("AAA");
    b = String::from("BBB");

    let c = select(&a, &b);
    a = String::from("aaa");
    b = String::from("bbb");

    println!("c {c}");
    println!("a {a}");
    println!("b {b}");
}

// fn select(name1: &str, name2: &str, choice: bool) -> &str {
//     if choice { name1 } else { name2 }
// }

//fn select<'a>(name1: &'a str, name2: &'a str) -> &'a str {
//    if name1 > name2 {
//        name1
//    } else {
//        name2
//    }
//}

fn select<'a>(name1: &str, name2: &str) -> &'a str {
    if name1 > name2 {
        "first"
    } else {
        "second"
    }
}
