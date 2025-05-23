fn number(num: i32) {
    match num {
        x if x > 30 => println!("{} is above 30", x),
        x if x > 20 => println!("{} is above 20", x),
        neg if neg < 0 => println!("{} is negative", neg),
        _ => println!("other: {num}"),
    }
}

fn main() {
    number(40);
    number(30);
    number(0);
    number(-7);
}
