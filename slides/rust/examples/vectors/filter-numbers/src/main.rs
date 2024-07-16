fn main() {
    let numbers: Vec<i32> = vec![-7, 0, 1, 2, 22, 23];
    println!("{:?}", &numbers);

    let same_numbers: Vec<i32> = numbers.iter().filter(|_number| true).cloned().collect();
    println!("{:?}", &same_numbers);

    let positive_numbers: Vec<i32> = numbers
        .iter()
        .filter(|number| number.is_positive())
        .cloned()
        .collect();
    println!("{:?}", &positive_numbers);

    let big_numbers: Vec<i32> = numbers
        .into_iter()
        .filter(|number| number > &&12)
        .collect();
    println!("{:?}", &big_numbers);
}
