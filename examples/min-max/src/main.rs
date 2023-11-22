fn main() {
    let x = 3;
    let y = 5;
    let min = std::cmp::min(x, y);
    println!("min: {}", min);

    let max = std::cmp::max(x, y);
    println!("max: {}", max);

    let x = "hello".to_string();
    let y = "world".to_string();

    let min = std::cmp::min(&x, &y);
    println!("min: {}", min);

    let max = std::cmp::max(x, y);
    println!("max: {}", max);
}
