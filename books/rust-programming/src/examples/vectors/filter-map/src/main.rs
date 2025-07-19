fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let big_numbers = numbers.iter().filter(|x| **x > 5).collect::<Vec<_>>();
    let doubled_numbers = big_numbers.iter().map(|x| *x * 2).collect::<Vec<_>>();

    println!("numbers:         {numbers:?}");
    println!("big_numbers:     {big_numbers:?}");
    println!("doubled_numbers: {doubled_numbers:?}");

    let doubled_numbers = numbers
        .iter()
        .filter(|x| **x > 5)
        .map(|x| *x * 2)
        .collect::<Vec<_>>();
    println!("doubled_numbers: {doubled_numbers:?}");

    let doubled_numbers = numbers
        .iter()
        .filter_map(|x| if *x > 5 { Some(*x * 2) } else { None })
        .collect::<Vec<_>>();
    println!("doubled_numbers: {doubled_numbers:?}");
}
