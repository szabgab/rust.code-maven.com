fn main() {
    let numbers: Vec<i32> = vec![-7, 0, 1, 2, 22, 23];
    println!("{:?}", &numbers);

    let same_numbers: Vec<i32> = numbers.iter().filter(|_number| true).cloned().collect();
    println!("{:?}", &same_numbers);

    // get vector of i32 references
    let positive_numbers = numbers
        .iter()
        .filter(|number| number.is_positive())
        .collect::<Vec<_>>();
    println!("{:?}", &positive_numbers);

    // get vector of i32
    let positive_numbers = numbers
        .iter()
        .filter(|number| number.is_positive())
        .cloned()
        .collect::<Vec<_>>();
    println!("{:?}", &positive_numbers);

    // get vector of i32 and move the numbers so we won't be able to use them again
    let big_numbers = numbers
        .into_iter()
        .filter(|number| number > &&12)
        .collect::<Vec<_>>();
    println!("{:?}", &big_numbers);
}
