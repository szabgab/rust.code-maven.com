fn main() {
    let (sum, diff) = calc(10, 5);
    println!("sum: {sum}  diff: {diff}");
}

fn calc(a: i32, b: i32) -> (i32, i32) {
    (a+b, a-b)
}
