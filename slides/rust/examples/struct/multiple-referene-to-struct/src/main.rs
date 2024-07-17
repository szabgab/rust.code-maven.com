#[derive(Debug)]
#[allow(dead_code)]
struct Something {
    number: i32,
    text: String,
    numbers: Vec<i32>,
}

fn main() {
    let a = Something {number: 2, text: String::from("blue"), numbers: vec![5, 6]};
    println!("{:?}", &a);

    let b = &a;
    println!("{:?}", &b);
    println!("{:?}", &a);
}

