fn main() {
    let mut a = String::new();
    let mut b = String::new();
    println!("a {a}");
    println!("b {b}");

    a = String::from("AAA");
    b = String::from("BBB");

    let c = select(&a, &b);
    //a = String::from("aaa");
    //b = String::from("bbb");

    println!("c {c}");
    println!("a {a}");
    println!("b {b}");
}

fn select<'a>(name1: &'a str, name2: &str) -> &'a str {
    if name1 > name2 {
        name1
    } else {
        "ab"
    }
}

// fn select<'a>(name1: &str, name2: &str) -> &'a str {
//     if name1 > name2 {
//         "first"
//     } else {
//         "second"
//     }
// }

// fn select(name1: &str, name2: &str) -> &'static str {
//     if name1 > name2 {
//         "first"
//     } else {
//         "second"
//     }
// }
