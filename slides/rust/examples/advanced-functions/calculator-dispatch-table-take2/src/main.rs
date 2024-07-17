use std::collections::HashMap;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let table: HashMap<char, fn(i32, i32) -> i32> = HashMap::from([
        ('+', add as fn(i32, i32) -> i32),
        ('*', multiply),
        ('-', |x, y| x - y),
        ('/', |x, y| x / y),
    ]);

    for op in ['+', '*', '-', '/'] {
        let res = table[&op](8, 2);
        println!("8 {op} 2 = {res}");
    }
}
