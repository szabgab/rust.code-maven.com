fn main() {
    let mut x = String::new();
    println!("{x}");
    x = String::from("before");
    println!("{x}");
    let c = select(&x);
    x = String::from("after");

    println!("{}", c);
    println!("{}", x);
}

fn select(text: &str) -> &'static str {
    if text > "abc" {
        "first"
    } else {
        "second"
    }
}

// fn select(text: &str) -> &str {
//     // fn select<'a>(text: &'a str) -> &'a str {  // the same as above
//     // fn select<'a, 'b>(text: &'a str) -> &'b str { // fully generic
//     // fn select<'b>(text: &str) -> &'b str { // specific lifetime
//     // fn select(text: &str) -> &'static str {
//         if text > "abc" {
//             "first"
//         } else {
//             "second"
//         }
//     }
