use std::collections::HashMap;

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn main() {
    let table = {
        let mut table: HashMap<char, fn(i32, i32) -> i32> = HashMap::new();
        table.insert('+', add);
        table.insert('*', multiply);
        table.insert('-', |x, y| x - y);
        table.insert('/', |x, y| x / y);
        table
    };

    for op in ['+', '*', '-', '/'] {
        let res = table[&op](8, 2);
        println!("8 {op} 2 = {res}");
    }
}
