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

//fn select(text: &str) -> &str {
//fn select<'a>(text: &'a str) -> &'a str {
//fn select<'a, 'b>(text: &'a str) -> &'b str {
fn select(text: &str) -> &'static str {
    if text > "abc" {
        "first"
    } else {
        "second"
    }
}

// fn select(text: &str) -> &str {
//     //fn select<'a>(text: &'a str) -> &'a str {
//     //fn select<'a, 'b>(text: &'a str) -> &'b str {
//     //fn select(text: &str) -> &'static str {
//         if text > "abc" {
//             text
//         } else {
//             "second"
//         }
//     }
    