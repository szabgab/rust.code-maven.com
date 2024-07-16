fn positive(number: &&i32) -> bool {
    number.is_positive()
}

fn main() {
    println!("{}", positive(&&3));
    println!("{}", positive(&&-3));
    let numbers: Vec<i32> = vec![-7, 0, 1, 2, 22, 23];
    println!("{:?}", &numbers);

    let positive_numbers: Vec<i32> = numbers
        .iter()
        .filter(|number| number.is_positive())
        .cloned()
        .collect();
    println!("{:?}", &positive_numbers);

    let positive_numbers: Vec<i32> = numbers.iter().filter(positive).cloned().collect();
    println!("{:?}", &positive_numbers);
}
