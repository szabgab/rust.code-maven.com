fn main() {
    let mut numbers:Vec<i8> = vec![];
    println!("{:?}", numbers);

    let input = "42";

    // names.push(input);
    // mismatched types

    let number = input.parse().unwrap();
    numbers.push(number);

    println!("{:?}", numbers);

    for num in numbers {
        println!("{}", num);
    }
}
