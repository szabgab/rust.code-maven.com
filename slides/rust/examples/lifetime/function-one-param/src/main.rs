fn main() {
    let mut a = String::new();
    println!("{a}");
    a = String::from("before");
    println!("{a}");

    let c = select(&a);
    a = String::from("after");

    println!("{}", c);
    println!("{}", a);
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
