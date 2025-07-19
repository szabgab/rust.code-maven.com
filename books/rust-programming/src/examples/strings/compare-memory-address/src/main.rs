fn main() {
    let text_a = "Hello, world!";
    let text_b = "Hello, world!";
    println!("{}", text_a == text_b);
    println!(
        "{}",
        std::ptr::addr_of!(text_a) == std::ptr::addr_of!(text_b)
    );
    println!("{:?}", &std::ptr::addr_of!(text_a));
    println!("{:?}", &std::ptr::addr_of!(text_b));
    println!();

    let text_a = String::from("Hello, world!");
    let text_b = String::from("Hello, world!");
    println!("{}", text_a == text_b);
    println!(
        "{}",
        std::ptr::addr_of!(text_a) == std::ptr::addr_of!(text_b)
    );
    println!("{:?}", &std::ptr::addr_of!(text_a));
    println!("{:?}", &std::ptr::addr_of!(text_b));
}
