fn main() {
    let n = 100;

    let mut text = String::from("x");
    for i in 0..n {
        text.push_str(&text.clone());
        println!("len: {} {}", i, text.len());
    }
}
