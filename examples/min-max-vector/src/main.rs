fn main() {
    let numbers: Vec<i32> = vec![23, 10, 78, 30];
    println!("numbers: {:?}", numbers);
    let min = numbers.iter().min().unwrap();
    println!("min: {}", min);

    let max = numbers.iter().max().unwrap();
    println!("max: {}", max);

    let other: Vec<i32> = vec![23];
    let empty: Vec<i32> = vec![];

    println!();
    for vec in vec![numbers, empty, other] {
        if vec.len() > 0 {
            let min = vec.iter().min().unwrap();
            println!("min: {}", min);
        } else {
            println!("empty");
        }
    }
}
